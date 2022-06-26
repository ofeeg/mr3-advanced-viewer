use iced::{button, Alignment, Button, Column, Element, Sandbox, Settings, Text, TextInput, Row, text_input};
mod mr3_data;
pub fn main() -> iced::Result {
    Viewer::run(Settings::default())
}



struct Viewer {
    value: String,
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
    //InputChanged(String)
}


impl Sandbox for Viewer {
    type Message = Message;
    fn new() -> Self {
	Viewer{
	    value: String::new(),
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
	    //Message::InputChanged(new_value) => {self.value = new_value;},
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

	let first_column = Column::new()
	    .padding(20)
	    .align_items(Alignment::Center)
	    .push(Text::new(((self.data.0.0)/10u16).to_string()))
	    .push(Text::new("LIF")).into();
	
	let second_column = Column::new()
	    .padding(20)
	    .align_items(Alignment::Center)
	    .push(Text::new(((self.data.0.1)/10u16).to_string()))
	    .push(Text::new("POW")).into();
	
	let third_column = Column::new()
	    .padding(20)
	    .align_items(Alignment::Center)
	    .push(Text::new(((self.data.0.2)/10u16).to_string()))
	    .push(Text::new("INT")).into();

	let fourth_column = Column::new()
	    .padding(20)
	    .align_items(Alignment::Center)
	    .push(Text::new(((self.data.0.3)/10u16).to_string()))
	    .push(Text::new("SPD")).into();

	let fifth_column = Column::new()
	    .padding(20)
	    .align_items(Alignment::Center)
	    .push(Text::new(((self.data.0.4)/10u16).to_string()))
	    .push(Text::new("DEF")).into();

	let sixth_column = Column::new()
	    .padding(20)
	    .align_items(Alignment::Center)
	    .push(Text::new(((self.data.0.5)/10u16).to_string()))
	    .push(Text::new("Lifespan")).into();
	
	let seventh_column = Column::new()
	    .padding(20)
	    .align_items(Alignment::Center)
	    .push(Text::new(((self.data.0.6)/10u16).to_string()))
	    .push(Text::new("InitialSpan")).into();

	let eigth_column = Column::new()
	    .padding(20)
	    .align_items(Alignment::Center)
	    .push(Text::new(((self.data.0.7)).to_string()))
	    .push(Text::new("Fatigue")).into();

	let ninth_column = Column::new()
	    .padding(20)
	    .align_items(Alignment::Center)
	    .push(Text::new(((self.data.0.8)).to_string()))
	    .push(Text::new("Stress")).into();

	let tenth_column = Column::new()
	    .padding(20)
	    .align_items(Alignment::Center)
	    .push(Text::new(((self.data.0.9)).to_string()))
	    .push(Text::new("Fear")).into();

	let eleventh_column = Column::new()
	    .padding(20)
	    .align_items(Alignment::Center)
	    .push(Text::new(((self.data.0.10)).to_string()))
	    .push(Text::new("Spoil")).into();

	let twelfth_column = Column::new()
	    .padding(20)
	    .align_items(Alignment::Center)
	    .push(Text::new(((self.data.0.11)).to_string()))
	    .push(Text::new("Form")).into();

	let thirteenth_column = Column::new()
	    .padding(20)
	    .align_items(Alignment::Center)
	    .push(Text::new(((self.data.0.12)).to_string()))
	    .push(Text::new("Protein")).into();

	let fourteenth_column = Column::new()
	    .padding(20)
	    .align_items(Alignment::Center)
	    .push(Text::new(((self.data.0.13)).to_string()))
	    .push(Text::new("Vitamin")).into();

	let fifteenth_column = Column::new()
	    .padding(20)
	    .align_items(Alignment::Center)
	    .push(Text::new(((self.data.0.14)).to_string()))
	    .push(Text::new("Mineral")).into();

	
	let sixteenth_column = Column::new()
	    .padding(20)
	    .align_items(Alignment::Center)
	    .push(Text::new(((self.data.1)).to_string()))
	    .push(Text::new("Money")).into();

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
    
    

