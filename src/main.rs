use iced::{button, Alignment, Button, Column, Element, Sandbox, Settings, Text, Row};
use mr3_data::MonMove;
mod mr3_data;
pub fn main() -> iced::Result {
    Viewer::run(Settings::default())
}

type ViewData = (
    (u16, u16, u16, u16, u16,
     u16, u16,
     u8, u8, u8, u8,
     u8, u8, u8, u8,
     u8
    ),
	u32,
	(
	    u8, u8, u8, u8, u8
	),
	(MonMove,
	MonMove,
	MonMove,
	 MonMove),
    [String;4]
);
lazy_static::lazy_static!{static ref MR3_D: std::sync::Mutex<ViewData> = std::sync::Mutex::new(((0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,69),
		  0,
		  (69, 69, 69, 69, 69),
			      ((0,0,0,0,0,0,0,0,0,0,0,0), (0,0,0,0,0,0,0,0,0,0,0,0), (0,0,0,0,0,0,0,0,0,0,0,0), (0,0,0,0,0,0,0,0,0,0,0,0)), ["QWERTYUIOPASDFGH".to_string(),"QWERTYUIOPASDFGH".to_string(),"QWERTYUIOPASDFGH".to_string(),"QWERTYUIOPASDFGH".to_string()]),
) ;}

struct Viewer {
    update_button: button::State,
    move_button: button::State,
    data: Rc<&'static MR3_D>,
}

struct ViewerSubM {
    data: Rc<&'static MR3_D>
}

#[derive(Debug, Clone)]
enum Message {
    UpdatePressed,
    MovePressed,
}


macro_rules! sr {
    ($s:expr) => {
	match $s{
	    0 => {"(E)"}
	    1 => {"(E)"}
	    2 => {"(E)"}
	    3 => {"(D)"}
	    4 => {"(D)"}
	    5 => {"(C)"}
	    6 => {"(C)"}
	    7 => {"(B)"}
	    8 => {"(B)"}
	    9 => {"(A)"}
	    _ => {"(?)"}
	}
    }
}


macro_rules! val {
    ($data:expr, $desc:literal) => {
	 Column::new()
	    .padding(20)
	    .align_items(Alignment::Center)
	    .push(Text::new($data))
	    .push(Text::new($desc)).into()
    };
}



use std::rc::Rc;
impl Sandbox for Viewer {
    type Message = Message; 
    fn new() -> Self {
	Viewer{
	    update_button: button::State::new(),
	    move_button: button::State::new(),
	    data: Rc::new(&MR3_D),
	}
	
    }
    fn title(&self) -> String {
        String::from("MR3 Advanced Viewer")
    }

    fn update(&mut self, message: Message)  {
        match message {
            Message::UpdatePressed => {
		*self.data.lock().unwrap() = match mr3_data::connect_to_mr3()
		{
		    Ok(it) =>  it,
		    Err(err) => panic!("{}", err),
		}
	    },
	    #[allow(unused_must_use)]
	    Message::MovePressed => {ViewerSubM::run(Settings::default());}
	}
    }

    fn view(&mut self) -> Element<Message> {
        let button_column = Column::new()
            .padding(20)
            .align_items(Alignment::Center)
            .push(
		Button::new(&mut self.update_button, Text::new("Update"))
		    .on_press(Message::UpdatePressed),
	    ).into();
	let button_column2 = Column::new()
            .padding(20)
            .align_items(Alignment::Center)
            .push(
		Button::new(&mut self.move_button, Text::new("Movelist"))
		    .on_press(Message::MovePressed),
	    ).into();

	let first_column = //val!(((self.data.lock().unwrap().0.0) / 10u16).to_string() + sr!(self.data.lock().unwrap().2.0), "LIF");
	val!(((match self.data.lock()
					       {
						   Ok(y) => y.0.0,
						   Err(err) => panic!("{}", err),
					       }
	)/10u16).to_string() + sr!(match self.data.lock(){
						   Ok(y) => y.2.0,
						   Err(err) => panic!("{}", err),}), "LIF");

	let second_column =
	//val!(((self.data.lock().unwrap().0.1) / 10u16).to_string() + sr!(self.data.lock().unwrap().2.1), "POW");
	val!(((match self.data.lock()
					       {
						   Ok(y) => y.0.1,
						   Err(err) => panic!("{}",err),
					       }
	)/10u16).to_string() + sr!(match self.data.lock(){
						   Ok(y) => y.2.1,
						   Err(err) => panic!("{}", err),}), "POW");
	//val!(((*self.data).0.1/10u16).to_string()+sr!((*self.data).2.1), "POW");
	let third_column = //val!(((self.data.lock().unwrap().0.2) / 10u16).to_string() + sr!(self.data.lock().unwrap().2.2), "INT");
	val!(((match self.data.lock()
					       {
						   Ok(y) => y.0.2,
						   Err(err) => panic!("{}", err),
					       }
	)/10u16).to_string() + sr!(match self.data.lock(){
						   Ok(y) => y.2.2,
						   Err(err) => panic!("{}", err),}), "INT");
	//val!(((*self.data).0.2/10u16).to_string()+sr!((*self.data).2.2), "INT");
	let fourth_column = //val!(((self.data.lock().unwrap().0.3) / 10u16).to_string() + sr!(self.data.lock().unwrap().2.3), "SPD");
	val!(((match self.data.lock()
					       {
						   Ok(y) => y.0.3,
						   Err(err) => panic!("{}", err),
					       }
	)/10u16).to_string() + sr!(match self.data.lock(){
						   Ok(y) => y.2.3,
						   Err(err) => panic!("{}", err),}), "SPD");
	//val!(((*self.data).0.3/10u16).to_string()+sr!((*self.data).2.3), "SPD");
	let fifth_column = //val!(((self.data.lock().unwrap().0.4) / 10u16).to_string() + sr!(self.data.lock().unwrap().2.4), "DEF");
	val!(((match self.data.lock()
					       {
						   Ok(y) => y.0.4,
						   Err(err) => panic!("{}", err),
					       }
	)/10u16).to_string() + sr!(match self.data.lock(){
						   Ok(y) => y.2.4,
						   Err(err) => panic!("{}", err),}), "DEF");
	//val!(((*self.data).0.4/10u16).to_string()+sr!((*self.data).2.4), "DEF");
	let sixth_column = //val!(((self.data.lock().unwrap().0.5) / 10u16).to_string(), "Lifespan");
	val!(((match self.data.lock()
				  {
				      Ok(y) => y.0.5,
				      Err(err) => panic!("{}", err),
				  }
	)/10u16).to_string(), "Lifespan");
	//val!(((*self.data).0.5/10u16).to_string(), "Lifespan");
	let seventh_column = //val!(((self.data.lock().unwrap().0.6) / 10u16).to_string(), "InitialSpan");
	val!(((match self.data.lock()
				  {
				      Ok(y) => y.0.6,
				      Err(err) => panic!("{}", err),
				  }
	)/10u16).to_string(), "InitialSpan");
	//val!(((*self.data).0.6/10u16).to_string(), "InitialSpan");
	let eigth_column = //val!(((self.data.lock().unwrap().0.7)).to_string(), "Fatigue");
	val!(((match self.data.lock()
				  {
				      Ok(y) => y.0.7,
				      Err(err) => panic!("{}", err),
				  }
	)).to_string(), "Fatigue");
	//val!(((*self.data).0.7).to_string(), "Fatigue");
	let ninth_column =  //val!(((self.data.lock().unwrap().0.8)).to_string(), "Stress");
	val!(((match self.data.lock()
				  {
				      Ok(y) => y.0.8,
				      Err(err) => panic!("{}", err),
				  }
	)).to_string(), "Stress");
	//val!(((*self.data).0.8).to_string(), "Stress");
	let tenth_column =  //val!(((self.data.lock().unwrap().0.9)).to_string(), "Fear");
	val!(((match self.data.lock()
				  {
				      Ok(y) => y.0.9,
				      Err(err) => panic!("{}", err),
				  }
	)).to_string(), "Fear");
	//val!(((*self.data).0.9).to_string(), "Fear");
	let eleventh_column = //val!(((self.data.lock().unwrap().0.10)).to_string(), "Spoil");
	val!(((match self.data.lock()
				  {
				      Ok(y) => y.0.10,
				      Err(err) => panic!("{}", err),
				  }
	)).to_string(), "Spoil");
	//val!(((*self.data).0.10).to_string(), "Spoil");
	let twelfth_column = //val!(((self.data.lock().unwrap().0.11)).to_string(),"Form");
	val!(((match self.data.lock()
				  {
				      Ok(y) => y.0.11,
				      Err(err) => panic!("{}", err),
				  }
	)).to_string(), "Form");
	//val!(((*self.data).0.11).to_string(),"Form");
	let thirteenth_column = //val!(((self.data.lock().unwrap().0.12)).to_string(),"Protein");
	val!(((match self.data.lock()
				  {
				      Ok(y) => y.0.12,
				      Err(err) => panic!("{}", err),
				  }
	)).to_string(), "Protein");
	//val!(((*self.data).0.12).to_string(),"Protein");
	let fourteenth_column = //val!(((self.data.lock().unwrap().0.13)).to_string(),"Vitamin");
	val!(((match self.data.lock()
				  {
				      Ok(y) => y.0.13,
				      Err(err) => panic!("{}", err),
				  }
	)).to_string(), "Vitamin");
	//val!(((*self.data).0.13).to_string(), "Vitamin");
	let fifteenth_column = //val!(((self.data.lock().unwrap().0.14)).to_string(),"Mineral");
	val!(((match self.data.lock()
				  {
				      Ok(y) => y.0.14,
				      Err(err) => panic!("{}", err),
				  }
	)).to_string(), "Mineral");
	//val!(((*self.data).0.14).to_string(), "Mineral");
	let seventeenth_column = //val!(mr3_data::determine_gr(self.data.lock().unwrap().0.11),"GrowthRate");
	val!(mr3_data::determine_gr(match self.data.lock()
				  {
				      Ok(y) => y.0.15,
				      Err(err) => panic!("{}", err),
				  }
	)
				      //(*self.data).0.15)
				      , "GrowthRate");
	let sixteenth_column = //val!(self.data.lock().unwrap().1.to_string(), "Money");
	val!((match self.data.lock()
				  {
				      Ok(y) => y.1,
				      Err(err) => panic!("{}", err),
				  }).to_string(),"Money");

	Column::with_children(vec!
			      [
				  button_column,
				  Row::with_children(vec![first_column, second_column, third_column, fourth_column,fifth_column]).into(),
				  Row::with_children(vec![sixth_column, seventh_column, seventeenth_column]).into(),
				  Row::with_children(vec![eigth_column, ninth_column,tenth_column, eleventh_column]).into(),
				  Row::with_children(vec![twelfth_column,thirteenth_column, fourteenth_column, fifteenth_column, sixteenth_column]).into(),
				  button_column2,
			      ]
			      ).into()
    }

}				      
    
    

fn express_move<'a>(m: mr3_data::MonMove, c: String) -> Row<'a, Message>
{
    let mut _s: String = "".to_string();
    _s.push_str(&c.as_str());
    let mov_name = val!(_s, "MoveName");
    //let mov_icon = val!(m.0, ""); useless for now
    let mov_lvl = val!(m.1.to_string(), "Lvl");
    let mov_maxlvl = val!(m.2.to_string(), "MaxLvl");
    let mov_type = val!(m.3.to_string(), "Type");
    let mov_xp = val!(m.4.to_string(), "XP");
    //let mov_slot = val!(m.5, "MoveSlot") useless for now
    let mov_cost = val!(m.6.to_string(), "Cost");
    let mov_acc = val!(m.7.to_string(), "Accuracy");
    let mov_crit = val!(m.8.to_string(), "Crit");
    let mov_dmg = val!(m.9.to_string(), "DMG");
    let mov_wit = val!(m.10.to_string(), "Withering");
    let mov_eff1 = val!(m.11.to_string(), "Effect1");
    //let mov_eff2 = val!(m.12.to_string(), "Effect2");
    //let mov_elem = val!(m.13.to_string(), "Element");
    Row::with_children(vec!
		       [
			   mov_name, mov_lvl, mov_maxlvl,
			   mov_type, mov_xp, mov_cost,
			   mov_acc, mov_crit, mov_dmg,
			   mov_wit, mov_eff1, //mov_eff2,
			   //mov_elem
		       ])
}


impl Sandbox for ViewerSubM {
    type Message = Message;
    fn new() -> Self
    {
	ViewerSubM{
	    data: Rc::new(&MR3_D),
	}
    }
    fn title(&self) -> String {
	String::from("Movelist")
    }
    fn update(&mut self, _message: Self::Message) {
	
    }
    fn view(&mut self) -> Element<Message>
    {
	let _1 = match self.data.lock()
					       {
						   Ok(y) => {y.4[0].clone()},
						   Err(err) => panic!("{}", err)
					       };
	let _2 = match self.data.lock()
					       {
						   Ok(y) => {y.4[1].clone()},
						   Err(err) => panic!("{}", err)
					       };

	let _3 = match self.data.lock()
					       {
						   Ok(y) => {y.4[2].clone()},
						   Err(err) => panic!("{}", err)
					       };

	let _4 = match self.data.lock()
					       {
						   Ok(y) => {y.4[3].clone()},
						   Err(err) => panic!("{}", err)
					       };
	
	Column::with_children(vec![
	    express_move(match self.data.lock()
				  {
				      Ok(y) => y.3.0,
				      Err(err) => panic!("{}", err),
				  },_1).into(),
	    express_move(match self.data.lock()
				  {
				      Ok(y) => y.3.1,
				      Err(err) => panic!("{}", err),
				  },_2).into(),
	    express_move(match self.data.lock()
				  {
				      Ok(y) => y.3.2,
				      Err(err) => panic!("{}", err),
				  },_3).into(),
	    express_move(match self.data.lock()
				  {
				      Ok(y) => y.3.3,
				      Err(err) => panic!("{}",err),
				  },_4).into()
	]).into()
    }
}

