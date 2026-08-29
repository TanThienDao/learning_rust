#[derive(Debug, PartialEq, Eq)]
enum ChannelType {
    Comedy,
    News,
    ProgrammingTutorials,
}
#[derive(Debug)]
struct TVChannel {
    name: String,
    channel_type: ChannelType,
}
fn main() {
    let channels = [
        TVChannel {
            name: String::from("CBS"),
            channel_type: ChannelType::News,
        },
        TVChannel {
            name: String::from("Rust Live"),
            channel_type: ChannelType::ProgrammingTutorials,
        },
        TVChannel {
            name: String::from("Comedy Central"),
            channel_type: ChannelType::Comedy,
        },
        TVChannel {
            name: String::from("CNN"),
            channel_type: ChannelType::News,
        },
        TVChannel {
            name: String::from("Rust TV"),
            channel_type: ChannelType::ProgrammingTutorials,
        },
    ];
    let good_channels = channels
        .iter()
        .filter(|channel| channel.channel_type == ChannelType::ProgrammingTutorials)
        .collect::<Vec<&TVChannel>>();

    println!("{:?}", good_channels.len() == channels.len());

    let good_channels = channels
        .iter()
        .find(|channel| channel.channel_type == ChannelType::ProgrammingTutorials);

    println!("{:?}", good_channels.is_some());

    let is_all_are_rust = channels
        .iter()
        .all(|channel| channel.channel_type == ChannelType::ProgrammingTutorials);

    println!("{:?}", is_all_are_rust);

    let any_are_rust = channels
        .iter()
        .any(|channel| channel.channel_type == ChannelType::ProgrammingTutorials);
    println!("{:?}", any_are_rust);
}
