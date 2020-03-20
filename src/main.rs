/*
 * The MIT License (MIT)
 *
 *               Copyright (c)  2020. Johannes Thorén, MIT License
 *
 *               Permission is hereby granted, free of charge, to any person obtaining a copy
 *               of this software and associated documentation files (the "Software"), to deal
 *               in the Software without restriction, including without limitation the rights
 *               to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 *               copies of the Software, and to permit persons to whom the Software is
 *               furnished to do so, subject to the following conditions:
 *
 *               The above copyright notice and this permission notice shall be included in
 *               all copies or substantial portions of the Software.
 *
 *               THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 *               IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 *               FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 *               AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 *               LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 *               OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 *               THE SOFTWARE.
 */

extern crate reqwest;

use std::fs::File;
use std::io;
use clap::*;

fn get_json(subreddit:&str, key:&str) -> json::JsonValue {
   let response = reqwest::get(&format!("https://www.reddit.com/r/{}/new.json?sort={}", subreddit, key))
       .expect("Could not make the request")
       .text().expect("could not read the text");

   json::parse(response.as_str()).unwrap()
}

fn get_post(json_data:json::JsonValue, post_index:usize) -> (String, String, String){

   let post_image_url = format!("{}", json_data["data"]["children"][post_index]["data"]["url"]);
   let post_title = format!("{}", json_data["data"]["children"][post_index]["data"]["title"]);
   let post_url = format!("https://reddit.com{}", json_data["data"]["children"][post_index]["data"]["permalink"]);

   (post_title, post_url, post_image_url)
}

fn get_filetype(post_image_url:&str) -> String {
   let split = post_image_url.split(".");
   let vec = split.collect::<Vec<&str>>() ;

   vec[vec.len() -1].to_string()
}


fn special_char_check (str_to_check:&mut String) -> String {
   let special_chars = vec!["\\", "/", "\"", "?", ":", "*", "<", ">", "|"];

   for char in special_chars {
      if str_to_check.contains(char) {
         str_to_check.remove(str_to_check.find(char).unwrap());
      }
   }

   str_to_check.parse().unwrap()
}

fn download_post(subreddit:&str, index:usize, key:&str) {
   let (post_title, post_url, post_image_url) = get_post(get_json(subreddit, key), index);
   let file_name = special_char_check(&mut format!("{}.{}", post_title, get_filetype(&post_image_url)));
   let out = File::create(file_name);

   match out {
      Ok(mut x) => {
         match io::copy(&mut reqwest::get(&post_image_url).unwrap(), &mut x) {
            Ok(_) => {
               println!("title : {}\nimage url : {}\npost url : {}\n\n", post_title, post_image_url, post_url)
            },
            Err(e) => { println!("\n\n\n{}", e) }
         }
      },
      Err(e) => {println!("\n\n\n{}", e)},
   };


}

fn main()  {

   let matches = App::new("dep-handler")
       .version("0.1")
       .author("Johannes T. <github.com/JohannesThoren>")
       .about("this is a small program that makes it possible to add dependencies through the command line")
       .arg(
          Arg::with_name("subreddit")
              .short("s")
              .long("sub")
              .takes_value(true)
              .value_name("subreddit")
              .help("takes the argument an uses it as a subreddit"))
       .arg(
          Arg::with_name("count")
              .short("c")
              .long("count")
              .takes_value(true)
              .value_name("count")
              .help("amount of images that you are going to download"))
       .arg(
      Arg::with_name("top")
          .short("t")
          .long("top")
          .takes_value(false)
          .help("sets sort key to top"))
       .arg(
          Arg::with_name("new")
              .short("n")
              .long("new")
              .takes_value(false)
              .help("sets sort key to new"))
       .get_matches();

   let mut sub = "dankmemes";
   let mut count = 1;
   let mut key = "none";

   if matches.is_present("subreddit") {
      sub = matches.value_of("subreddit").unwrap();
   }
   if matches.is_present("count") {
      count = matches.value_of("count").unwrap().parse().unwrap();
   }
   if matches.is_present("new") {
      key = "new"
   }
   if matches.is_present("top") {
      key = "top"
   }


   for c in 0..count{
      download_post(sub, c, key)
   }

}