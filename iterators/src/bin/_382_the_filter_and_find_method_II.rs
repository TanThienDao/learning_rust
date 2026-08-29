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

    let good_channels: Vec<&TVChannel> = channels
        .iter()
        .filter(|chanel| chanel.channel_type == ChannelType::ProgrammingTutorials)
        .collect();

    println!("{:?}", good_channels);
    let good_channels: Vec<String> = channels
        .iter()
        .filter(|chanel| chanel.channel_type == ChannelType::ProgrammingTutorials)
        .map(|chanel| chanel.name.clone())
        .collect();
    println!("{:?}", good_channels);

    let good_channels = channels
        .iter()
        .find(|channel| channel.channel_type == ChannelType::ProgrammingTutorials);

    println!("{:?}", good_channels);

    match good_channels {
        Some(channel) => println!("Great Choice to watch: {:?}", channel),
        None => println!("There are no rust programing tutorials channels available"),
    }

    let good_channels = channels
        .iter()
        .rfind(|channel| channel.channel_type == ChannelType::ProgrammingTutorials);

    println!("{:?}", good_channels);

    let good_channels = channels
        .iter()
        .filter(|channel| channel.channel_type == ChannelType::Comedy)
        .nth(2);

    match good_channels {
        Some(channel) => println!("Great Choice to watch in second list: {:?}", channel),
        None => println!("There are no comedy channels available"),
    }
}
