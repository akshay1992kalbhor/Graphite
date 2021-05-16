use graphite_proc_macros::AsMessage;
use std::fmt::Display;

trait AsMessage: Sized + Into<Message> + Send + Sync + PartialEq<Message> + Display {
	//trait AsMessage: Sized + Send + Sync {
	//trait AsMessage: Sized + Send + Sync + Into<Message> + Display + PartialEq<Message> {
	//trait AsMessage: Sized + Send + Sync + Into<Message> + Display {
	fn name(&self) -> String;
	fn suffix(&self) -> &'static str;
	fn prefix() -> String;
	fn get_discriminant(&self) -> MessageDiscriminant;
}

#[derive(AsMessage, PartialEq, Clone)]
#[message(Message, Message, Child)]
enum Message {
	Child(Child),
}

#[derive(AsMessage, PartialEq, Clone)]
#[message(Message, Message, Child)]
pub enum Child {
	Foo,
	Document(DocumentMessage),
}

#[derive(AsMessage, PartialEq, Clone)]
#[message(Message, Child, Document)]
pub enum DocumentMessage {
	Foo,
	Bar,
}

fn main() {
	println!("Hello, world!");
}
