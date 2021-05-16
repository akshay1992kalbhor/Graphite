use graphite_proc_macros::MessageImpl;
use std::fmt::Display;

trait AsMessage: Sized + Into<Message> + Send + Sync + PartialEq<Message> + Display + Clone {
	//trait AsMessage: Sized + Send + Sync {
	//trait AsMessage: Sized + Send + Sync + Into<Message> + Display + PartialEq<Message> {
	//trait AsMessage: Sized + Send + Sync + Into<Message> + Display {
	fn name(&self) -> String;
	fn suffix(&self) -> &'static str;
	fn prefix() -> String;
	fn get_discriminant(&self) -> MessageDiscriminant;
}

#[derive(MessageImpl, PartialEq, Clone)]
#[message(Message, Message, Child)]
enum Message {
	Foo(usize),
	#[child]
	Child(Child),
}

#[derive(MessageImpl, PartialEq, Clone)]
#[message(Message, Message, Child)]
pub enum Child {
	Foo(usize),
	#[child]
	Document(DocumentMessage),
}

#[derive(MessageImpl, PartialEq, Clone)]
#[message(Message, Child, Document)]
pub enum DocumentMessage {
	Foo,
	Bar(usize),
}

fn main() {
	println!("Hello, world!");
}
