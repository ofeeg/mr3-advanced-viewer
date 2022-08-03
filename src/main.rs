use iced::{button,  Container, container, Alignment, Button, Column, Element, Sandbox, Settings, Text, Row, Color, Background,};
use mr3_data::MonMove;
mod mr3_data;
pub fn main() -> iced::Result {
    Viewer::run(Settings::default())
}


struct StatRate{
    rate: String,
}

impl container::StyleSheet for StatRate
{
    fn style(&self) -> container::Style {
	container::Style
	{
	    text_color: Some(Color::WHITE),
	    background: match self.rate.as_str(){
		"(E)" => Some(Background::Color(Color::from_rgb(1f32, 0f32, 0f32))),
		"(D)" => Some(Background::Color(Color::from_rgb(0.75f32, 0.5f32, 0f32))),
		"(C)" => Some(Background::Color(Color::from_rgb(0f32, 0.5f32, 0.25f32))),
		"(B)" => Some(Background::Color(Color::from_rgb(0f32, 0.75f32, 0.15f32))),
		"(A)" => Some(Background::Color(Color::from_rgb(0f32, 1f32, 0f32))),
		_ => Some(Background::Color(Color::from_rgb(0f32, 0f32, 0f32)))
	    },
	    border_radius: 10f32,
	    border_width: 10f32,
	    border_color: Color::TRANSPARENT,
	}
    }
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
	    .push(Text::new($desc))
    };
    /*($data:expr, $desc:literal, $highlight:literal) => {
	Column::new()
	    .padding(20)
	    .align_items(Alignment::Center)
	    .push(Text::new($data))
	    .push(Text::new($desc))
	    .push(Text::new($highlight.to_string()))
    };*/
}
macro_rules! monget
{
    ($self:ident, $loc:tt, $loc2:tt) => {
	match $self.data.lock()
	{
	    Ok(y) => {y.$loc.$loc2},
	    Err(err) => panic!("{}", err),
	}
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

	let first_column =
	    Container::new(
		val!((monget!(self, 0, 0)
				  /10u16).to_string() + sr!(monget!(self,2,0)), "LIF"))
	    .style(
		StatRate{rate: sr!(monget!(self,2,0)).to_string()}).into();
	
	let second_column =
	    Container::new(val!(((monget!(self,0,1))/10u16).to_string() +
				sr!(monget!(self,2,1)), "POW")).style(StatRate{rate: sr!(monget!(self,2,1)).to_string()}).into();
	let third_column = 
	    Container::new(val!(((monget!(self,0,2))/10u16).to_string() +
			    sr!(monget!(self,2,2)),"INT")).style(StatRate{rate: sr!(monget!(self,2,2)).to_string()}).into();
	let fourth_column = Container::new(val!(((monget!(self,0,3))/10u16).to_string() +
						sr!(monget!(self,2,3)),"SPD")).style(StatRate{rate: sr!(monget!(self,2,3)).to_string()}).into();
	let fifth_column = Container::new(val!(((monget!(self,0,4))/10u16).to_string() +
			    sr!(monget!(self,2,4)),"DEF")).style(StatRate{rate: sr!(monget!(self,2,4)).to_string()}).into();
	
	let sixth_column = val!(((match self.data.lock()
				  {
				      Ok(y) => y.0.5,
				      Err(err) => panic!("{}", err),
				  }
	)/10u16).to_string(), "Lifespan").into();

	let seventh_column = 
	    val!(((match self.data.lock()
		   {
		       Ok(y) => y.0.6,
		       Err(err) => panic!("{}", err),
		   }
	    )/10u16).to_string(), "InitialSpan").into();
	let eigth_column = 
	    val!(((match self.data.lock()
		   {
		       Ok(y) => y.0.7,
		       Err(err) => panic!("{}", err),
		   }
	    )).to_string(), "Fatigue").into();
	let ninth_column =  
	    val!(((match self.data.lock()
		   {
		       Ok(y) => y.0.8,
		       Err(err) => panic!("{}", err),
		   }
	    )).to_string(), "Stress").into();
	let tenth_column =  
	    val!(((match self.data.lock()
		   {
		       Ok(y) => y.0.9,
		       Err(err) => panic!("{}", err),
		   }
	    )).to_string(), "Fear").into();
	let eleventh_column =
	    val!(((match self.data.lock()
		   {
		       Ok(y) => y.0.10,
		       Err(err) => panic!("{}", err),
		   }
	    )).to_string(), "Spoil").into();
	
	let twelfth_column = 
	    val!(((match self.data.lock()
		   {
		       Ok(y) => y.0.11,
		       Err(err) => panic!("{}", err),
		   }
	    )).to_string(), "Form").into();
	let thirteenth_column = 
	    val!(((match self.data.lock()
		   {
		       Ok(y) => y.0.12,
		       Err(err) => panic!("{}", err),
		   }
	    )).to_string(), "Protein").into();
	let fourteenth_column = 
	    val!(((match self.data.lock()
		   {
		       Ok(y) => y.0.13,
		       Err(err) => panic!("{}", err),
		   }
	    )).to_string(), "Vitamin").into();
	let fifteenth_column = 
	    val!(((match self.data.lock()
		   {
		       Ok(y) => y.0.14,
		       Err(err) => panic!("{}", err),
		   }
	    )).to_string(), "Mineral").into();
	let seventeenth_column = 
	    val!(mr3_data::determine_gr(match self.data.lock()
					{
					    Ok(y) => y.0.15,
					    Err(err) => panic!("{}", err),
					}
	    )
		 , "GrowthRate").into();
	let sixteenth_column =
	    val!((match self.data.lock()
		  {
		      Ok(y) => y.1,
		      Err(err) => panic!("{}", err),
		  }).to_string(),"Money").into();
	
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
    let mov_name = val!(_s, "MoveName").into();
    //let mov_icon = val!(m.0, ""); useless for now
    let mov_lvl = val!(m.1.to_string(), "Lvl").into();
    let mov_maxlvl = val!(m.2.to_string(), "MaxLvl").into();
    let mov_type = val!(m.3.to_string(), "Type").into();
    let mov_xp = val!(m.4.to_string(), "XP").into();
    //let mov_slot = val!(m.5, "MoveSlot") useless for now
    let mov_cost = val!(m.6.to_string(), "Cost").into();
    let mov_acc = val!(m.7.to_string(), "Accuracy").into();
    let mov_crit = val!(m.8.to_string(), "Crit").into();
    let mov_dmg = val!(m.9.to_string(), "DMG").into();
    let mov_wit = val!(m.10.to_string(), "Withering").into();
    let mov_eff1 = val!(m.11.to_string(), "Effect1").into();
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

