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
use std::borrow::BorrowMut;

fn get_json(subreddit:&str) -> json::JsonValue {
   let response = reqwest::get(format!("https://www.reddit.com/r/{}/new.json?sort={}", subreddit, "new").as_str())
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

fn download_post(subreddit:&str, index:usize) {
   let (post_title, post_url, post_image_url) = get_post(get_json(subreddit), index);
   let mut file_name = special_char_check(format!("{}.{}", post_title, get_filetype(&post_image_url)).borrow_mut());
   let mut out = File::create(file_name).expect(format!("error! could not create file with name {}", post_title).as_ref());
   io::copy(&mut reqwest::get(&post_image_url).unwrap(), &mut out).expect("error! could not write data");
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
       .get_matches();

   let mut sub = "unket";
   let mut count = 1;

   if matches.is_present("subreddit") {
      sub = matches.value_of("subreddit").unwrap();
   }
   if matches.is_present("count") {
      count = matches.value_of("count").unwrap().parse().unwrap();
   }

   download_post(sub, count);

}