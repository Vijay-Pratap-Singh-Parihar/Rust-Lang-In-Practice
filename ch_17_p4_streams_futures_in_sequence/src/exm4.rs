use trpl::{Stream, StreamExt, ReceiverStream};
use std::time::Duration;
use std::pin::pin;

//This example combines everything. We'll have two different conveyor belts (one for "news," one for "ads") and merge them into one. We'll also manage the flow to keep things clean.

// News kitchen (har second ek news bhejta hai)
fn get_news_feed() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();
    trpl::spawn_task(async move {
        for i in 1..=3 {
            trpl::sleep(Duration::from_secs(1)).await;
            tx.send(format!("News: Story #{i}")).unwrap();
        }
    });
    ReceiverStream::new(rx)
}

// Ad kitchen (har 300ms ek ad bhejta hai)
fn get_ad_feed() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();
    trpl::spawn_task(async move {
        for i in 1.. { // Infinite ads
            trpl::sleep(Duration::from_millis(300)).await;
            if tx.send(i).is_err() { break; }
        }
    });
    ReceiverStream::new(rx)
}

fn main() {
    trpl::run(async {
        println!("\n--- Example 3: Merging Two Belts ---");

        let news_stream = get_news_feed();
        let ad_stream = get_ad_feed();

        // Ad stream (`u32`) ko `String` mein badal rahe hain taaki type match ho.
        let formatted_ad_stream = ad_stream.map(|ad_id| format!("Ad: Visit site #{ad_id}!"));
        
        // Dono belts ko merge kar rahe hain. Ab jo bhi pehle aayega, dikhega.
        let merged_stream = news_stream.merge(formatted_ad_stream);

        // Hum final stream ko manage kar rahe hain:
        // `.take(10)`: Hum sirf pehle 10 items hi lenge, taaki infinite ad loop ruk jaaye.
        let mut final_stream = pin!(merged_stream.take(10));
        
        while let Some(item) = final_stream.next().await {
            println!("Feed item: {item}");
        }
        println!("Feed finished after 10 items!");
    });
}