use rusqlite::{Connection, Result};
use std::error::Error;
use serde::Deserialize;
use mysql::*;
use mysql::prelude::*;

//Need a runtime for async stuff
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
   let settings = get_settings()?;

   println!("The sqlite database file: {}", settings.sqlitedb);
   let conn = Connection::open(&settings.sqlitedb)?;

   println!("Creating sqlite tables...");
   create_tables(&conn)?;

   println!("Connecting to mysql.. ");
   let mpool = get_mysqlpool(&settings)?;

   println!("Querying for threads...");
   let threads = mysql_getthreads(&mpool)?;
   println!("Querying for posts...");
   let posts = mysql_getposts(&mpool)?;
   println!("Querying for bans...");
   let bans = mysql_getbans(&mpool)?;

   println!("Inserting {} threads...", threads.len());
   sqlite_insertthreads(&conn, &threads)?;
   println!("Inserting {} posts...", posts.len());
   sqlite_insertposts(&conn, &posts)?;
   println!("Inserting {} bans...", bans.len());
   sqlite_insertbans(&conn, &bans)?;

   println!("All complete!");
   Ok(())
}

#[derive(Debug, Deserialize)]
pub struct MySettings {
   pub sqlitedb : String,
   pub mysqlconstring : String
}

#[derive(Debug, Deserialize)]
pub struct Post {
   pub pid: i32,
   pub tid: i32,
   pub created: mysql::chrono::NaiveDateTime, //chrono::DateTime<chrono::Utc>,
   pub content: String,
   pub options: String,
   pub ipaddress: String,
   pub username: Option<String>,
   pub tripraw: Option<String>,
   pub image: Option<String>
}


#[derive(Debug, Deserialize)]
pub struct Thread {
   pub tid: i32,
   pub created: mysql::chrono::NaiveDateTime,
   pub subject: String,
   pub deleted: bool,
   pub hash: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct Ban {
   pub range: String,
   pub created: mysql::chrono::NaiveDateTime,
   pub note: Option<String>
}

//Retrieve the raw config object that houses the settings
fn get_settings_raw() -> Result<config::Config, config::ConfigError> {
   let mut settings = config::Config::default();
   settings.merge(config::File::with_name("Settings"))?;
   Ok(settings) //transfer ownership
}

//Retrieve the far-more-usable settings object
fn get_settings() -> Result<MySettings, config::ConfigError> {
   let settings = get_settings_raw()?;
   let myset : MySettings = settings.try_into()?;
   Ok(myset)
}

//Create the sqlite tables and structures that mimic the kland stuff
fn create_tables(conn: &Connection) -> Result<(), Box<dyn Error>> {
   conn.execute(
       "create table if not exists bans (
          range text unique,
          created text not null,
          note text)", 
       [],)?;
   conn.execute(
       "create table if not exists threads (
          tid integer primary key,
          created text not null,
          subject text not null,
          deleted int not null,
          hash text)", 
       [],)?;
   conn.execute(
       "create table if not exists posts (
          pid integer primary key,
          tid integer not null,
          created text not null,
          content text not null,
          options text not null,
          ipaddress text not null,
          username text,
          tripraw text,
          image text)", 
       [],)?;
   conn.execute(
       "create index if not exists idx_posts_tid on posts(tid)", [],)?;
   Ok(())
}

fn get_mysqlpool(settings: &MySettings) -> Result<Pool, Box<dyn Error>> {
   let mopts = Opts::from_url(&settings.mysqlconstring)?;
   let mpool = Pool::new(mopts)?;
   Ok(mpool)
}

fn mysql_getbans(pool: &Pool) -> Result<Vec<Ban>, Box<dyn Error>> {
   let mut conn = pool.get_conn()?;
   let bans = conn
      .query_map(
            "SELECT `range`, created, `note` from bans",
            |(range, created, note)| {
               Ban { range, created, note }
            },
         )?;
   Ok(bans)
}

fn mysql_getthreads(pool: &Pool) -> Result<Vec<Thread>, Box<dyn Error>> {
   let mut conn = pool.get_conn()?;
   let threads = conn
      .query_map(
            "SELECT tid, created, subject, deleted, hash from threads",
            |(tid, created, subject, deleted, hash)| {
               Thread { tid, created, subject, deleted, hash }
            },
         )?;
   Ok(threads)
}

fn mysql_getposts(pool: &Pool) -> Result<Vec<Post>, Box<dyn Error>> {
   let mut conn = pool.get_conn()?;
   let posts = conn
      .query_map(
            "SELECT pid, tid, created, content, options, ipaddress, username, 
             tripraw, image from posts",
            |(pid, tid, created, content, options, ipaddress, username, tripraw, image)| {
               Post { pid, tid, created, content, options, ipaddress, username,
                      tripraw, image }
            },
         )?;
   Ok(posts)
}

fn sqlite_insertthreads(conn: &Connection, threads: &Vec<Thread>) -> Result<(), Box<dyn Error>> {
   let mut stm = conn.prepare("INSERT INTO threads (tid, created, subject, deleted, hash) VALUES 
            (?1, ?2, ?3, ?4, ?5)")?;
   for t in threads.iter() {
      stm.execute(rusqlite::params![t.tid, t.created, t.subject, t.deleted, t.hash])?;
    }
    Ok(())
}

fn sqlite_insertposts(conn: &Connection, posts: &Vec<Post>) -> Result<(), Box<dyn Error>> {
   let mut stm = conn.prepare("INSERT INTO posts (pid, tid, created, content, options, 
      ipaddress, username, tripraw, image) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)")?;
   for p in posts.iter() {
      stm.execute(rusqlite::params![p.pid, p.tid, p.created, p.content,
         p.options, p.ipaddress, p.username, p.tripraw, p.image])?;
    }
    Ok(())
}

fn sqlite_insertbans(conn: &Connection, bans: &Vec<Ban>) -> Result<(), Box<dyn Error>> {
   let mut stm = conn.prepare("INSERT INTO bans (range, created, note) VALUES 
            (?1, ?2, ?3)")?;
   for b in bans.iter() {
      stm.execute(rusqlite::params![b.range, b.created, b.note])?;
    }
    Ok(())
}
