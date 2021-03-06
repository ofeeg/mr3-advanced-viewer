use process_memory::{Memory, DataMember,TryIntoProcessHandle, ProcessHandle};
use sysinfo::{PidExt, ProcessExt, System, SystemExt};

#[cfg(windows)]
fn pcsx2_handle(pid: sysinfo::Pid) -> ProcessHandle{pid.as_u32().try_into_process_handle().unwrap()}

#[cfg(not(windows))]

fn pcsx2_handle(pid: sysinfo::Pid) -> ProcessHandle {i32::from(pid).try_into_process_handle().unwrap()}
pub fn connect_to_mr3() ->(
    (u16, u16, u16, u16, u16,
     u16, u16,
     u8, u8, u8, u8,
     u8, u8, u8, u8,
     u8
    ),
    u32,
    (u8, u8, u8, u8, u8)
)
    
{
    let mut pcsx2_pid: sysinfo::Pid = sysinfo::Pid::from(0); //= Pid::from(1);
    let mut sys = System::new_all();
    sys.refresh_all();
    for (pid, process) in sys.processes() {
	//if process.name() == "pcsx2.exe".to_string() {println!("Connected: [{}] {}", pid, process.name());}
	if process.name() == "pcsx2.exe".to_string() { pcsx2_pid = *pid; }
    }
    let handle = pcsx2_handle(pcsx2_pid);
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
    (
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
	)
    )
    
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
