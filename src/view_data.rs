use iced::{futures::{future::ok, channel::mpsc, lock::Mutex},Error,  Subscription};
use iced_native::subscription;
use process_gaddrs_derive::ProcessGaddrs;
use process_gaddrs::ProcessGaddrs;
use process_memory::{Memory, TryIntoProcessHandle, ProcessHandle};
use crate::data_member::DataMember;
#[allow(unused_imports)]
use sysinfo::{PidExt, ProcessExt, System, SystemExt};

pub const M_L_S: usize = 0;
pub const M_D_S: usize = 1;
pub const M_I_S: usize = 2;
pub const M_S_S: usize = 3;
pub const M_P_S: usize = 4;
pub const LIFESPAN: usize = 5;
pub const INITSPAN: usize = 6;
pub const FATIGUE: usize = 0;
pub const STRESS: usize = 1;
pub const FEAR: usize = 2;
pub const SPOIL: usize = 3;
pub const FORM: usize = 4;
pub const PROTEIN: usize = 5;
pub const VITAMIN: usize = 6;
pub const MINERAL: usize = 7;
pub const M_G_R: usize = 0;
pub const M_P_R: usize = 1;
pub const M_S_R: usize = 2;
pub const M_I_R: usize = 3;
pub const M_D_R: usize = 4;
pub const M_L_R: usize = 5;

pub type MonMove = (u8, u8, u8, u8, u16, u8, u8, u8, u8, u16, u16, u16);
#[derive(Debug, Clone)]
pub struct MonData
{
    pub battle_stats: [u16;7],
    pub invisible_stats: [u8;8],
    pub stat_rates: [u8;6]
}

#[derive(Debug,  Clone)]
pub struct ViewData {
    pub stat_data: MonData,
    pub move_data: (MonMove,
		MonMove,
		MonMove,
		MonMove),
    pub move_names: [String;4],
    pub player_money: u32,
	
}

impl ViewData
{
    pub fn new(sd: MonData,
	   md: (MonMove,
		MonMove,
		MonMove,
		MonMove),
	   mn: [String;4],
	   pm: u32
    ) -> Self
    {
	ViewData { stat_data: sd, move_data: md, move_names: mn, player_money: pm }
    }   
}

lazy_static::lazy_static!{pub static ref MR3_D: Mutex<ViewData> = Mutex::new(ViewData::new(MonData{battle_stats: [0,0,0,0,0,69,69],
												   invisible_stats: [0,0,0,0,0,0,0,0],
												   stat_rates: [69, 69, 69, 69, 69, 69]},
											   ((0,0,0,0,0,0,0,0,0,0,0,0), (0,0,0,0,0,0,0,0,0,0,0,0), (0,0,0,0,0,0,0,0,0,0,0,0), (0,0,0,0,0,0,0,0,0,0,0,0)), [("QWERTYUIOPASDFGH").to_string(),("QWERTYUIOPASDFGH").to_string(),("QWERTYUIOPASDFGH").to_string(),("QWERTYUIOPASDFGH").to_string()], 0)) ;}


#[derive(Debug,Clone)]
pub enum Progress
{
    Started(mpsc::Sender<u8>),
    Finished,
    Errored,
}


#[derive(Debug)]
pub enum State
{
    Start,
    Ready(mpsc::Receiver<u8>)
}

#[cfg(windows)]
fn pcsx2_handle(pid: sysinfo::Pid) -> ProcessHandle{pid.as_u32().try_into_process_handle().unwrap()}


#[cfg(not(windows))]
fn pcsx2_handle(pid: sysinfo::Pid) -> ProcessHandle {i32::from(pid).try_into_process_handle().unwrap()}

#[derive(ProcessGaddrs)]
struct MoveOffsets{
    valid_addresses: Vec<u32>,
}
fn get_pcsx2() -> ProcessHandle
{
    let mut pcsx2_pid: sysinfo::Pid = sysinfo::Pid::from(0);
    let mut sys = System::new_all();
    sys.refresh_all();
    for (pid, process) in sys.processes() {
	//if process.name() == "pcsx2.exe".to_string() {println!("Connected: [{}] {}", pid, process.name());}
	if process.name() == "pcsx2.exe".to_string() { pcsx2_pid = *pid; }
    }
    pcsx2_handle(pcsx2_pid)
}

//static PCSX2_HANDLE: ProcessHandle = get_pcsx2();
static MOVE1_ADDR : u32 = 0x203841E0;
static MOVE2_ADDR : u32 = 0x20384228;
static MOVE3_ADDR : u32 = 0x20384270;
static MOVE4_ADDR : u32 = 0x203842B8;


async fn connect(handle: ProcessHandle)
{
    let move_offsets: [u32;15] = 
	[16,24,25,26,27,28,31,32,33,34,35,36,38,41,54];
    let mut move1_addrs = MoveOffsets::_new(15);
    move1_addrs.init_data(0x203841E0, move_offsets);
    let mut move2_addrs = MoveOffsets::_new(15);
    move2_addrs.init_data(0x20384228, move_offsets);
    let mut move3_addrs = MoveOffsets::_new(15);
    move3_addrs.init_data(0x20384270, move_offsets);
    let mut move4_addrs = MoveOffsets::_new(15);
    move4_addrs.init_data(0x203842B8, move_offsets);
    let b_s: [u16;7];
    let i_s: [u8;8];
    let s_r: [u8;6];
    let m_d: (MonMove, MonMove, MonMove,MonMove);
    let m_n: [String;4];
    m_d = (
	extract_move(move1_addrs.valid_addresses, handle),
	extract_move(move2_addrs.valid_addresses, handle),
	extract_move(move3_addrs.valid_addresses, handle),
	extract_move(move4_addrs.valid_addresses, handle),
    );
    b_s = [ //lif def int spd pow
	DataMember::new_offset(handle, vec![0x20_38_41_70]).read().unwrap(),
	DataMember::new_offset(handle, vec![0x20_38_41_6E]).read().unwrap(),
	DataMember::new_offset(handle, vec![0x20_38_41_6C]).read().unwrap(),
	DataMember::new_offset(handle, vec![0x20_38_41_6A]).read().unwrap(),
	DataMember::new_offset(handle, vec![0x20_38_41_68]).read().unwrap(),
	DataMember::new_offset(handle, vec![0x20_38_41_84]).read().unwrap(), //Lifespan
	DataMember::new_offset(handle, vec![0x20_38_41_86]).read().unwrap(), //Initspan
    ];
    i_s = [
	DataMember::new_offset(handle, vec![0x20_38_41_89]).read().unwrap(), //Fatigue
	DataMember::new_offset(handle, vec![0x20_38_41_8A]).read().unwrap(), //Stress
	DataMember::new_offset(handle, vec![0x20_38_41_D4]).read().unwrap(), //Fear
	DataMember::new_offset(handle, vec![0x20_38_41_D5]).read().unwrap(), //Spoil
	DataMember::new_offset(handle, vec![0x20_38_41_8B]).read().unwrap(), //Form
	DataMember::new_offset(handle, vec![0x20_38_41_A4]).read().unwrap(), //Protein
	DataMember::new_offset(handle, vec![0x20_38_41_A5]).read().unwrap(), //Vitamin
	DataMember::new_offset(handle, vec![0x20_38_41_A6]).read().unwrap(), //Mineral
    ];
    s_r = [
	DataMember::new_offset(handle, vec![0x20_38_41_32]).read().unwrap(), //GrowthRate
	DataMember::new_offset(handle, vec![0x20_38_41_91]).read().unwrap(),// pow, spd, int,def,lif
	DataMember::new_offset(handle, vec![0x20_38_41_92]).read().unwrap(),
	DataMember::new_offset(handle, vec![0x20_38_41_93]).read().unwrap(),
	DataMember::new_offset(handle, vec![0x20_38_41_94]).read().unwrap(),
	DataMember::new_offset(handle, vec![0x20_38_41_95]).read().unwrap(),

    ];
    m_n = [extract_string(MOVE1_ADDR, handle),extract_string(MOVE2_ADDR, handle),extract_string(MOVE3_ADDR, handle),extract_string(MOVE4_ADDR, handle)];
    
	*MR3_D.lock().await = ok::<ViewData, Error>(ViewData::new(MonData{battle_stats: b_s,
						    invisible_stats: i_s,
						    stat_rates: s_r},			
				 m_d,
					   m_n, 0)).await.expect("Update data")
}


pub fn connect_process() -> Subscription<Progress>
{
    struct Connect;
    unsafe impl Send for Connect{};
    unsafe impl Sync for Connect{};
    subscription::unfold(std::any::TypeId::of::<Connect>(), State::Start, |state| async move
    {
	match state
	{
	    State::Start => {
		let (sender, receiver) = mpsc::channel(100);
		(Some(Progress::Started(sender)),
		State::Ready(receiver))
	    }
	    State::Ready(_input) => {
		connect(get_pcsx2()).await;
		(Some(Progress::Finished), State::Start)
	    }
	}
    })
}


fn extract_move(addrs: Vec<u32>, handle: ProcessHandle) -> MonMove
{
    let data: MonMove =
	(
	    DataMember::new_offset(handle, vec![addrs[2] as usize]).read().unwrap(),
	    DataMember::new_offset(handle, vec![addrs[3] as usize]).read().unwrap(),
	    DataMember::new_offset(handle, vec![addrs[4] as usize]).read().unwrap(),
	    DataMember::new_offset(handle, vec![addrs[5] as usize]).read().unwrap(),
	    DataMember::new_offset(handle, vec![addrs[6] as usize]).read().unwrap(),
	    DataMember::new_offset(handle, vec![addrs[8] as usize]).read().unwrap(),
	    DataMember::new_offset(handle, vec![addrs[9] as usize]).read().unwrap(),
	    DataMember::new_offset(handle, vec![addrs[10] as usize]).read().unwrap(),
	    DataMember::new_offset(handle, vec![addrs[11] as usize]).read().unwrap(),
	    DataMember::new_offset(handle, vec![addrs[12] as usize]).read().unwrap(),
	    DataMember::new_offset(handle, vec![addrs[13] as usize]).read().unwrap(),
	    DataMember::new_offset(handle, vec![addrs[14] as usize]).read().unwrap(),
	);
    data
}


fn extract_string(addr: u32, handle: ProcessHandle) -> String
{
    let mut c: String = "".to_string();
    for i in 0..15
    {
	match DataMember::<char>::new_offset(handle, vec![(addr+i) as usize]).read()
	{
	    Ok(chr) => {if (chr as u8) == 255{break};c.push(mr3char(chr as u8));
	    }
	Err(e) => eprintln!("Read error {}", e), 
	}
    }
    //println!("{}", c);
    c
}
//"Pokorin" should be 0F-28-24-28-2B-22-27 hex 19 is a
fn mr3char(mut d: u8) -> char {
    if d <= 25 {d += 65; char::from(d)}
    else if d <= 56 {d+=71; char::from(d)}
    else{' '}    
}

unsafe impl Send for ViewData{}
unsafe impl Sync for ViewData{}
