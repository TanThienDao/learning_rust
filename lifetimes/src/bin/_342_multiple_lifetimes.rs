#[derive(Debug)]
struct TravelPlan<'a, 'b> {
    from: &'a str,
    to: &'b str,
}

fn main() {
    let from = String::from("PortLand");

    let plan = {
        let to = String::from("Bangor");
        let travel_plan = TravelPlan {
            from: &from,
            to: &to,
        };

        travel_plan.from
    };

    println!("{}", plan);

    let plan2 = figure_out_ending_point(&from);
    println!("{}", plan2);
}

fn figure_out_ending_point(from: &str) -> &str {
    let to = String::from("PortLand_ to_ Bangor");
    let travel_plan = TravelPlan {
        from: &from,
        to: &to,
    };
    travel_plan.from
}
