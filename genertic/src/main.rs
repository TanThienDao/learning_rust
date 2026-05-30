mod test;
#[derive(Debug)]
enum DigitalContent {
    AudioFile,
    VideoFile
}

#[derive(Debug)]
struct ChatMessage<T> {
    content: T,
    time: String,
}
impl ChatMessage<DigitalContent> {
    fn consume_entertaiment(&self){
        println!("Watching the AudioFile: {:?}", self.content);
    }
}
impl<T> ChatMessage<T> {
    fn retrieve_time(&self) -> String {
        println!("The message was sent at: {}", self.time);
        self.time.clone()
    }
}
fn main() {
    let chat_1 = ChatMessage{
        content:"string slice",
        time:"2017-12-13".to_string()
    };
    let chat_2 = ChatMessage{
        content: String::from("String"),
        time: "2017-12-13".to_string()
    };
    let chat_3 = ChatMessage{
        content: DigitalContent::AudioFile,
        time: "2017-12-13".to_string()
    };
    chat_3.consume_entertaiment();


    chat_1.retrieve_time();
    chat_2.retrieve_time();
    chat_3.retrieve_time();
}





