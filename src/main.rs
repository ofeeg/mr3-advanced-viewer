use iced::{button, Alignment, Button, Column, Element, Sandbox, Settings, Text, Row};
mod mr3_data;
pub fn main() -> iced::Result {
    Viewer::run(Settings::default())
}



struct Viewer {
    connect_button: button::State,
    data: (
    (u16, u16, u16, u16, u16,
     u16, u16,
     u8, u8, u8, u8,
     u8, u8, u8, u8
    ),
    u32
) ,
}

#[derive(Debug, Clone)]
enum Message {
    ConnectPressed,
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

impl Sandbox for Viewer {
    type Message = Message;
    fn new() -> Self {
	Viewer{
	    connect_button: button::State::new(),
	    data:((0,0,0,0,0,0,0,0,0,0,0,0,0,0,0),0) 
	}
    }

    fn title(&self) -> String {
        String::from("MR3 Advanced Viewer")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ConnectPressed => {self.data = mr3_data::connect_to_mr3()},
        }
    }

    fn view(&mut self) -> Element<Message> {
        let button_column = Column::new()
            .padding(20)
            .align_items(Alignment::Center)
            .push(
		Button::new(&mut self.connect_button, Text::new("Connect"))
		    .on_press(Message::ConnectPressed),
	    ).into();

	let first_column = val!((self.data.0.0/10u16).to_string(), "LIF");
	
	let second_column = val!((self.data.0.1/10u16).to_string(), "POW");
	
	let third_column = val!((self.data.0.2/10u16).to_string(), "INT");

	let fourth_column = val!((self.data.0.3/10u16).to_string(), "SPD");

	let fifth_column = val!((self.data.0.4/10u16).to_string(), "DEF");

	let sixth_column = val!((self.data.0.5/10u16).to_string(), "Lifespan");
    
	let seventh_column = val!((self.data.0.6/10u16).to_string(), "InitialSpan");

	let eigth_column = val!((self.data.0.7).to_string(), "Fatigue");

	let ninth_column = val!((self.data.0.8).to_string(), "Stress");

	let tenth_column = val!((self.data.0.9).to_string(), "Fear");

	let eleventh_column = val!((self.data.0.10).to_string(), "Spoil");

	let twelfth_column = val!((self.data.0.11).to_string(),"Form");

	let thirteenth_column = val!((self.data.0.12).to_string(),"Protein");

	let fourteenth_column = val!((self.data.0.13).to_string(), "Vitamin");
	let fifteenth_column = val!((self.data.0.14).to_string(), "Mineral");
	
	let sixteenth_column = val!((self.data.1).to_string(),"Money");

	Column::with_children(vec!
			      [
				  button_column,
				  Row::with_children(vec![first_column, second_column, third_column, fourth_column,fifth_column, sixth_column, seventh_column]).into(),
				  Row::with_children(vec![eigth_column, ninth_column,tenth_column, eleventh_column, twelfth_column,thirteenth_column, fourteenth_column, fifteenth_column, sixteenth_column
				  ]
				  ).into()
			      ]
			      ).into()
    }

}				      
    
    

