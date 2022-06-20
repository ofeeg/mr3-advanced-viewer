//extern crate libc;
//extern crate process_memory;
use process_memory::{Memory, DataMember, Pid, TryIntoProcessHandle};

pub fn connect_to_mr3() ->(
    (u16, u16, u16, u16, u16,
     u16, u16,
     u8, u8, u8, u8),
    u32
)
    
{
    
    let mut pid_addr: i32 = 0;
    let mut pcsx2_pid: Pid = Pid::from(1);
    #[cfg(not(target_os = "linux"))]{pcsx2_pid = Pid::from(pid_addr as u32);}
    #[cfg(target_os = "linux")]{let pcsx2_pid = Pid::from(pid_addr);}
    let handle = (pcsx2_pid).try_into_process_handle().unwrap();
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
    let player_money = DataMember::new_offset(handle, vec![0x20_37_BE_AC]);
    println!("Connected!");
    (
	(mons_lif.read().unwrap(), mons_pow.read().unwrap(), mons_int.read().unwrap(),
	 mons_spd.read().unwrap(), mons_def.read().unwrap(), mons_lifespan.read().unwrap(),
	 mons_initspan.read().unwrap(), mons_fatigue.read().unwrap(),
	 mons_stress.read().unwrap(), mons_spoil.read().unwrap(), mons_fear.read().unwrap()),
	player_money.read().unwrap()
    )
}
