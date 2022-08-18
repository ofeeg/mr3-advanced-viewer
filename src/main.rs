use iced::{Application, Background, Color, container, Element,  Settings, Command, executor, Text,   futures::executor::block_on,  Subscription, Container, Alignment, Column, Row, button, Button, Length, widget::scrollable};
mod view_data;
use lazy_static::__Deref;
use view_data::{ViewData, connect_process, MR3_D, M_L_S, M_L_R, M_P_R, M_P_S, M_I_S, M_I_R, M_S_S, M_S_R, M_D_S, M_D_R, LIFESPAN, INITSPAN, FATIGUE, SPOIL, FORM, PROTEIN, VITAMIN, MINERAL, M_G_R, STRESS, FEAR, MonMove};

pub fn main() -> iced::Result
{
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
	    .padding(10)
	    .align_items(Alignment::Center)
	    .push(Text::new($data))
	    .push(Text::new($desc))
    };
}

struct Viewer {
    data : ViewData,
    move_button: button::State,
    shrink_button: button::State,
    show_moves: bool,
    scrollbar: scrollable::State
}

#[derive(Debug, Clone)]
enum Message{
    Update(view_data::Progress),
    MovesPressed,
    ShrinkMPressed,
}

impl Application for Viewer
{
    type Message = Message;
    type Executor = executor::Default;
    type Flags = ();
    
    fn new(_flags: ()) -> (Viewer, Command<Message>)
    {
	(
	    Viewer{
		data:  MR3_D.try_lock().expect("Ask for data").deref().clone(),
		move_button: button::State::new(),
		shrink_button: button::State::new(),
		show_moves: false,
		scrollbar: scrollable::State::new()
		/*scrollbar: scrollable::Scrollable::new(&mut self.scrollbar)
		    .width(Length::Fill)
		    .height(Length::Units(100))
		    .style(style::Theme)
		    .push(Space::with_height(Length::Units(800)))
		  */  
	    
	    },
	    Command::none()
	)
    }
    fn title(&self) -> String{String::from("multi-threading")}

    fn subscription(&self) -> Subscription<Message> {
	connect_process().map(Message::Update)
    }
    fn update(&mut self, message: Message) -> Command<Message>
    {
	match message
	{
	    Message::Update(progress) => {
		match progress {
		    view_data::Progress::Started(_s) =>{},
		    view_data::Progress::Finished => {block_on(async {self.data  = MR3_D.lock().await.deref().clone()});},
		    view_data::Progress::Errored => {},
		}
	    },
	    Message::MovesPressed => {self.show_moves = true;}
	    Message::ShrinkMPressed => {self.show_moves = false;}
	}
	Command::none()
    }

    fn view(&mut self) -> Element<Message> {
	let move_button = Column::new()
	    .padding(20)
	    .align_items(Alignment::Center)
	    .push(Button::new(&mut self.move_button, Text::new("Moves"))
		  .on_press(Message::MovesPressed),
		  ).into();
	let mon_lif = Container::new(val!(((self.data.stat_data.battle_stats[M_L_S])/10u16).to_string()
			    + sr!(self.data.stat_data.stat_rates[M_L_R]), "LIF"))
	    .style(StatRate{rate: (sr!(self.data.stat_data.stat_rates[5]).to_string())}).into();
	let mon_pow = Container::new(val!(((self.data.stat_data.battle_stats[M_P_S])/10u16).to_string()
			    + sr!(self.data.stat_data.stat_rates[M_P_R]), "POW"))
	    .style(StatRate{rate: (sr!(self.data.stat_data.stat_rates[M_P_R]).to_string())}).into();
	let mon_int = Container::new(val!(((self.data.stat_data.battle_stats[M_I_S])/10u16).to_string()
			    + sr!(self.data.stat_data.stat_rates[M_I_R]), "INT"))
	    .style(StatRate{rate: (sr!(self.data.stat_data.stat_rates[M_I_R]).to_string())}).into();
	let mon_spd = Container::new(val!(((self.data.stat_data.battle_stats[M_S_S])/10u16).to_string()
			    + sr!(self.data.stat_data.stat_rates[M_S_R]), "SPD"))
	    .style(StatRate{rate: (sr!(self.data.stat_data.stat_rates[M_S_R]).to_string())}).into();
	let mon_def = Container::new(val!(((self.data.stat_data.battle_stats[M_D_S])/10u16).to_string()
			    + sr!(self.data.stat_data.stat_rates[M_D_R]), "DEF"))
	    .style(StatRate{rate: (sr!(self.data.stat_data.stat_rates[M_D_R]).to_string())}).into();
	let mon_lifespan = val!(((self.data.stat_data.battle_stats[LIFESPAN])/10u16).to_string(), "Lifespan").into();
	let mon_initspan = val!(((self.data.stat_data.battle_stats[INITSPAN])/10u16).to_string(), "Initial Lifespan").into();
	let mon_gr = val!(((self.data.stat_data.invisible_stats[M_G_R])/10u8).to_string(), "Growth Rate").into();
	let mon_fatigue =  val!(((self.data.stat_data.invisible_stats[FATIGUE])/10u8).to_string(), "Fatigue").into();
	let mon_stress = val!(((self.data.stat_data.invisible_stats[STRESS])/10u8).to_string(), "Stress").into();
	let mon_spoil =  val!(((self.data.stat_data.invisible_stats[SPOIL])/10u8).to_string(), "Spoil").into();
	let mon_fear = val!(((self.data.stat_data.invisible_stats[FEAR])/10u8).to_string(), "Fear").into();
	let mon_form =  val!(((self.data.stat_data.invisible_stats[FORM])/10u8).to_string(), "Form").into();
	let mon_protein =  val!(((self.data.stat_data.invisible_stats[PROTEIN])/10u8).to_string(), "Protein").into();
	let mon_vitamin =  val!(((self.data.stat_data.invisible_stats[VITAMIN])/10u8).to_string(), "Vitamin").into();
	let mon_mineral = val!(((self.data.stat_data.invisible_stats[MINERAL])/10u8).to_string(), "Mineral").into();
	
	let special_column =
	    if self.show_moves == true {
		Row::new().push(Button::new(&mut self.shrink_button, Text::new("Shrink"))
				.on_press(Message::ShrinkMPressed))
.push(
		Row::new().push(scrollable::Scrollable::new(&mut self.scrollbar)
				.width(Length::Fill)
				.height(Length::Units(400))
				.style(ScrollableX)
				.padding(5)
				.push(
				    Column::with_children(vec![
					express_move(self.data.move_data.0, self.data.move_names[0].to_string()).into(),
					express_move(self.data.move_data.1, self.data.move_names[1].to_string()).into(),
					express_move(self.data.move_data.2, self.data.move_names[2].to_string()).into(),
					express_move(self.data.move_data.3, self.data.move_names[3].to_string()).into(),
				    ])))
		    )
	    }
	else{Row::new()};
	
	Column::with_children(vec![
	    Row::with_children(vec![move_button]).into(),
	    Row::with_children(vec![mon_lif, mon_pow, mon_int, mon_spd, mon_def]).into(),
	    Row::with_children(vec![
		mon_lifespan, mon_initspan, mon_gr, mon_form
	    ]).into(),
	    Row::with_children(vec![
		mon_protein, mon_vitamin, mon_mineral
	    ]).into(),
	    Row::with_children(vec![
		mon_fatigue, mon_stress, mon_spoil, mon_fear
	    ]).into(),
	    special_column.into()
	    // Row::new()
	    // 	.spacing(20)
	    // 	.height(Length::Units(100))
	    // 	.align_items(Alignment::Center)
	    // 	.push(special_column)
	    // 	.into()
	]).into()

    }
    
}

fn express_move<'a>(m: MonMove, c: String) -> Row<'a, Message>
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
			   mov_name, mov_type, mov_cost,
			   Row::with_children(vec![
			       mov_lvl, mov_maxlvl, mov_xp,
			   ]).into(),
			   Row::with_children(vec![
			       mov_dmg, mov_acc, mov_crit, mov_wit
			   ]).into(),
			   Row::with_children(vec![
			       mov_eff1
			   ]).into()
			   
		       ]).padding(5)
}

pub struct ScrollableX;

impl scrollable::StyleSheet for ScrollableX {
    fn active(&self) -> scrollable::Scrollbar {
        scrollable::Scrollbar {
            background: SURFACE.into(),
            border_radius: 2.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            scroller: scrollable::Scroller {
                color: ACTIVE,
                border_radius: 2.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
        }
    }
    
    fn hovered(&self) -> scrollable::Scrollbar {
        let active = self.active();
	
        scrollable::Scrollbar {
            background: Color { a: 0.5, ..SURFACE }.into(),
            scroller: scrollable::Scroller {
                color: HOVERED,
                ..active.scroller
            },
            ..active
        }
    }
    
    fn dragging(&self) -> scrollable::Scrollbar {
        let hovered = self.hovered();
	
        scrollable::Scrollbar {
            scroller: scrollable::Scroller {
                color: Color::from_rgb(0.85, 0.85, 0.85),
                ..hovered.scroller
            },
            ..hovered
        }
    }
}
const SURFACE: Color = Color::from_rgb(
    0x40 as f32 / 255.0,
    0x44 as f32 / 255.0,
    0x4B as f32 / 255.0,
);

const ACTIVE: Color = Color::from_rgb(
    0x72 as f32 / 255.0,
    0x89 as f32 / 255.0,
    0xDA as f32 / 255.0,
);

const HOVERED: Color = Color::from_rgb(
    0x67 as f32 / 255.0,
    0x7B as f32 / 255.0,
    0xC4 as f32 / 255.0,
);
