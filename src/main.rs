use iced::{button, Alignment, Button, Column, Element, Sandbox, Settings, Text, Row,};
//use process_memory::DataMember;
use std::default::Default;
mod mr3_data;
pub fn main() -> iced::Result {
    //let data: MR3Data = MR3Data{data: mr3_data::connect_to_mr3()};
    Viewer::run(Settings::default())
}

#[derive(Default,Debug, Clone)]
struct MR3Data{
  data:(
    (u16, u16, u16, u16, u16,
     u16, u16,
     u8, u8, u8, u8),
    u32
),
}

#[derive(Default, Debug, Clone)]
struct Viewer {
    value: u16,
    connect_button: button::State,
    data: MR3Data,
}
#[derive(Debug, Clone)]
enum Message {
    ConnectPressed
}


impl Sandbox for Viewer {
    type Message = Message;
    fn new() -> Self {
        Self::default()	    
    }

    fn title(&self) -> String {
        String::from("MR3 Advanced Viewer")
    }

    fn update(&mut self, message: Message) {
	self.data =  MR3Data{data: mr3_data::connect_to_mr3()};
        match message {
            Message::ConnectPressed => {self.value = self.data.data.0.0/10},
        }
    }

    fn view(&mut self) -> Element<Message> {
        let first_column = Column::new()
            .padding(20)
            .align_items(Alignment::Center)
            .push(
		Button::new(&mut self.connect_button, Text::new("Connect"))
		    .on_press(Message::ConnectPressed),
	    )
            .push(Text::new(self.value.to_string())).into();
    
	let second_column = Column::new()
	    .padding(20)
	    .align_items(Alignment::Center)
	    .push(Text::new(((self.data.data.0.1)/10u16).to_string()))
	    .push(Text::new("POW")).into();
	
	let third_column = Column::new()
	    .padding(20)
	    .align_items(Alignment::Center)
	    .push(Text::new(((self.data.data.0.2)/10u16).to_string()))
	    .push(Text::new("INT")).into();

	let fourth_column = Column::new()
	    .padding(20)
	    .align_items(Alignment::Center)
	    .push(Text::new(((self.data.data.0.3)/10u16).to_string()))
	    .push(Text::new("SPD")).into();

	let fifth_column = Column::new()
	    .padding(20)
	    .align_items(Alignment::Center)
	    .push(Text::new(((self.data.data.0.4)/10u16).to_string()))
	    .push(Text::new("DEF")).into();

	let sixth_column = Column::new()
	    .padding(20)
	    .align_items(Alignment::Center)
	    .push(Text::new(((self.data.data.0.5)/10u16).to_string()))
	    .push(Text::new("Lifespan")).into();
	
	let seventh_column = Column::new()
	    .padding(20)
	    .align_items(Alignment::Center)
	    .push(Text::new(((self.data.data.0.6)/10u16).to_string()))
	    .push(Text::new("InitialSpan")).into();

	let eigth_column = Column::new()
	    .padding(20)
	    .align_items(Alignment::Center)
	    .push(Text::new(((self.data.data.0.7)).to_string()))
	    .push(Text::new("Fatigue")).into();

	let ninth_column = Column::new()
	    .padding(20)
	    .align_items(Alignment::Center)
	    .push(Text::new(((self.data.data.0.8)).to_string()))
	    .push(Text::new("Stress")).into();

	let tenth_column = Column::new()
	    .padding(20)
	    .align_items(Alignment::Center)
	    .push(Text::new(((self.data.data.0.9)).to_string()))
	    .push(Text::new("Fear")).into();

	let eleventh_column = Column::new()
	    .padding(20)
	    .align_items(Alignment::Center)
	    .push(Text::new(((self.data.data.0.10)).to_string()))
	    .push(Text::new("Spoil")).into();

	let twelfth_column = Column::new()
	    .padding(20)
	    .align_items(Alignment::Center)
	    .push(Text::new(((self.data.data.1)).to_string()))
	    .push(Text::new("Money")).into();

	Column::with_children(vec!
			      [
				  Row::with_children(vec![first_column, second_column, third_column, fourth_column,fifth_column, sixth_column, seventh_column]).into(),
				  Row::with_children(vec![eigth_column, ninth_column,tenth_column, eleventh_column, twelfth_column
				  ]
				  ).into()
			      ]
			      ).into()
			      
				      
    }
    
}
