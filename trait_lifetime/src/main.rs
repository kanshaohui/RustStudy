use trait_lifetime::{largest, Summary};
use trait_lifetime;
fn main() {
    //use largest function.
    let num_list = vec![55, 34, 25, 100, 64];
    let largest_num = largest::largest_num(&num_list);
    println!("The largest num is {}", largest_num);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let largest_char = largest::largest_char(&char_list);
    println!("The largest char is {}", largest_char);

    let num_list = vec![33, 44, 55, 66, 77, 11, 12, 15, 32];
    let largest_num = largest::largest_all(&num_list);
    println!("The largest <T> num is {}", largest_num);

    let char_list = vec!['z', 'k', 'y', 'a'];
    let largest_char = largest::largest_all(&char_list);
    println!("The largest <T> char is {}", largest_char);

    let loca = trait_lifetime::Point {
        x: 1,
        y: 3,
    };
    println!("loca.x = {}, loca.y = {}", loca.x, loca.y);

    let newtweet = trait_lifetime::Tweet{
        username: String::from("horse_book"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", newtweet.summarize());

    let article = trait_lifetime::NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };
    println!("1 new Article : {}", article.summarize());

    trait_lifetime::notify(article);

    let pair1 = trait_lifetime::Pair::new(5, 9);
    pair1.cmp_display();

    let x = 33;
    let y = 70;
    println!("The longest test is {}", largest::longest(&x ,&y));

    let novel = String::from("Call me Ismael. Some years ago ...");
    let first_sentence = novel.split('.')
        .next()
        .expect("找不到.");
    let expert = trait_lifetime::ImportantExpert{
        part: first_sentence,
    };
    println!("first sentence is : {}", expert.part);
    println!("expert's leve is {}", expert.level());
    println!("expert's annoucement is {}", expert.announce_and_return_part("Read a book"));

    println!("The longest test is {}", largest::longest_with_annoucement(&x ,&y, "Announce!"));

    println!("Hello, world!");
}
