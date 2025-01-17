use rand_agents::user_agent;

fn main() {
    let agent = user_agent();
    println!("Random User Agent: {}", agent);
}
