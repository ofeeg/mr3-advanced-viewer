use process_memory::{Memory, DataMember,TryIntoProcessHandle, ProcessHandle};
#[allow(unused_imports)]
use sysinfo::{PidExt, ProcessExt, System, SystemExt};
use std::sync::Mutex;
use lazy_static::lazy_static;
use process_gaddrs_derive::ProcessGaddrs;
use process_gaddrs::ProcessGaddrs;

#[cfg(windows)]
fn pcsx2_handle(pid: sysinfo::Pid) -> ProcessHandle{pid.as_u32().try_into_process_handle().unwrap()}

#[cfg(not(windows))]
fn pcsx2_handle(pid: sysinfo::Pid) -> ProcessHandle {i32::from(pid).try_into_process_handle().unwrap()}

#[derive(ProcessGaddrs)]
struct MoveOffsets{
    valid_addresses: Vec<u32>,
}

pub type MonMove = (u8, u8, u8, u8, u16, u8, u8, u8, u8, u16, u16, u16);

const NAMES_SIZE: usize = 16;
lazy_static!(pub static ref NAMES: Mutex<[String;4]> = Mutex::new([String::with_capacity(NAMES_SIZE),String::with_capacity(NAMES_SIZE),String::with_capacity(NAMES_SIZE),String::with_capacity(NAMES_SIZE)]););

static MOVE1_ADDR : u32 = 0x203841E0;
static MOVE2_ADDR : u32 = 0x20384228;
static MOVE3_ADDR : u32 = 0x20384270;
static MOVE4_ADDR : u32 = 0x203842B8;

pub fn connect_to_mr3() ->Result<(
    (u16, u16, u16, u16, u16,
     u16, u16,
     u8, u8, u8, u8,
     u8, u8, u8, u8,
     u8
    ),
    u32,
    (u8, u8, u8, u8, u8),
    (MonMove,
    MonMove,
    MonMove,
     MonMove),
    [String;4]
), Box<dyn std::error::Error>>
    
{
    let mut pcsx2_pid: sysinfo::Pid = sysinfo::Pid::from(0); //= Pid::from(1);
    let mut sys = System::new_all();
    sys.refresh_all();
    for (pid, process) in sys.processes() {
	//if process.name() == "pcsx2.exe".to_string() {println!("Connected: [{}] {}", pid, process.name());}
	if process.name() == "pcsx2.exe".to_string() { pcsx2_pid = *pid; }
    }
    *NAMES.lock()? = ["".to_string(),"".to_string(),"".to_string(),"".to_string()];
    let handle = pcsx2_handle(pcsx2_pid);
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
    let mon_move1: MonMove = extract_move(move1_addrs.valid_addresses, handle);
    let mon_move2: MonMove = extract_move(move2_addrs.valid_addresses, handle);
    let mon_move3: MonMove = extract_move(move3_addrs.valid_addresses, handle);
    let mon_move4: MonMove = extract_move(move4_addrs.valid_addresses, handle);
    let mons_lif = DataMember::new_offset(handle, vec![0x20_38_41_70]);
    let mons_def = DataMember::new_offset(handle, vec![0x20_38_41_6E]);
    let mons_int = DataMember::new_offset(handle, vec![0x20_38_41_6C]);
    let mons_spd = DataMember::new_offset(handle, vec![0x20_38_41_6A]);
    let mons_pow = DataMember::new_offset(handle, vec![0x20_38_41_68]);
    let mons_lifespan = DataMember::new_offset(handle, vec![0x20_38_41_84]);
    let mons_initspan = DataMember::new_offset(handle, vec![0x20_38_41_86]);
    let mons_fatigue = DataMember::new_offset(handle, vec![0x20_38_41_89]);
    let mons_stress = DataMember::new_offset(handle, vec![0x20_38_41_8A]);
    let mons_fear = DataMember::new_offset(handle, vec![0x20_38_41_D4]);
    let mons_spoil = DataMember::new_offset(handle, vec![0x20_38_41_D5]);
    let mons_form = DataMember::new_offset(handle, vec![0x20_38_41_8B]);
    let mons_prot = DataMember::new_offset(handle, vec![0x20_38_41_A4]);
    let mons_vita = DataMember::new_offset(handle, vec![0x20_38_41_A5]);
    let mons_mine = DataMember::new_offset(handle, vec![0x20_38_41_A6]);
    let mons_rate = DataMember::new_offset(handle, vec![0x20_38_41_32]);
    let pow_rate = DataMember::new_offset(handle, vec![0x20_38_41_91]);
    let spd_rate = DataMember::new_offset(handle, vec![0x20_38_41_92]);
    let int_rate = DataMember::new_offset(handle, vec![0x20_38_41_93]);
    let def_rate = DataMember::new_offset(handle, vec![0x20_38_41_94]);
    let lif_rate = DataMember::new_offset(handle, vec![0x20_38_41_95]);
    let player_money = DataMember::new_offset(handle, vec![0x20_37_BE_AC]);
    (*NAMES.lock()?)[0].push_str(&extract_string(MOVE1_ADDR, handle));
    let mov_name1 = (*NAMES.lock()?)[0].chars().collect();
    (*NAMES.lock()?)[1].push_str(&extract_string(MOVE2_ADDR, handle));
    let mov_name2 = (*NAMES.lock()?)[1].chars().collect();
    (*NAMES.lock()?)[2].push_str(&extract_string(MOVE3_ADDR, handle));
    let mov_name3 = (*NAMES.lock()?)[2].chars().collect();
    (*NAMES.lock()?)[3].push_str(&extract_string(MOVE4_ADDR, handle));
    let mov_name4 = (*NAMES.lock()?)[3].chars().collect();
    Ok((
	(mons_lif.read().unwrap(), mons_pow.read().unwrap(), mons_int.read().unwrap(),
	 mons_spd.read().unwrap(), mons_def.read().unwrap(), mons_lifespan.read().unwrap(),
	 mons_initspan.read().unwrap(), mons_fatigue.read().unwrap(),
	 mons_stress.read().unwrap(), mons_spoil.read().unwrap(), mons_fear.read().unwrap(),
	 mons_form.read().unwrap(), mons_prot.read().unwrap(),  mons_vita.read().unwrap(), mons_mine.read().unwrap(),
	 mons_rate.read().unwrap()
	),
	player_money.read().unwrap(),
	(
	    lif_rate.read().unwrap(), pow_rate.read().unwrap(), int_rate.read().unwrap(),
		spd_rate.read().unwrap(), def_rate.read().unwrap()
	),
	(mon_move1, mon_move2, mon_move3, mon_move4),
	[mov_name1,mov_name2,mov_name3,mov_name4]
    ))
    
}
pub fn determine_gr(gr: u8) -> String{
    match gr{
	0 => {"(Early Growth(Strong))".to_string()}
	1 => {"(Early Growth(Mild))".to_string()}
	2 => {"(Standard)".to_string()}
	3 => {"(Late Growth(Mild))".to_string()}
	4 => {"(Late Growth(Strong))".to_string()}
	5 => {"Sustainable".to_string()}
	_ => {"Invalid".to_string()}
    }
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
