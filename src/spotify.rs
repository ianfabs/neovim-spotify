extern crate reqwest;
extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate strum;
extern crate strum_macros;
extern crate dotenv;
extern crate url;

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use strum_macros::{Display, EnumString};
use dotenv::dotenv;
use url::Url;
use reqwest::Client;
use rand::{self, Rng};

use std::collections::HashMap;
use std::env;
use std::io::Read;
use std::str::FromStr;
use std::string::ToString;

pub trait SpotifyAPI {
    fn current_song(&self) -> String;
    fn play(&self);
    fn pause(&self);
    fn play_pause(&self);
    fn next(&self);
    fn previous(&self);
    fn login(&self);
}

pub struct Spotify;

//impl SpotifyAPI for Spotify {
//    fn new() {
//        //TODO: Make this do stuff
//    }
//}

pub mod auth {
    const SPOTIFY_AUTH_URL: &str = "https://accounts.spotify.com/authorize";
    const SPOTIFY_TOKEN_URL: &str = "https://accounts.spotify.com/api/token";
    pub fn generate_random_string(length: usize) -> String {
		rand::thread_rng()
			.sample_iter(&rand::distributions::Alphanumeric)
			.take(length)
			.collect()
	}

    #[derive(EnumString, Serialize, Deserialize, Display, Debug, Clone, PartialEq)]
    pub enum Scope {
        #[strum(serialize = "user-read-recently-played")]
        UserReadRecentlyPlayed,
        #[strum(serialize = "user-top-read")]
        UserTopRead,

        #[strum(serialize = "user-library-modify")]
        UserLibraryModify,
        #[strum(serialize = "user-library-read")]
        UserLibraryRead,

        #[strum(serialize = "playlist-read-private")]
        PlaylistReadPrivate,
        #[strum(serialize = "playlist-modify-public")]
        PlaylistModifyPublic,
        #[strum(serialize = "playlist-modify-private")]
        PlaylistModifyPrivate,
        #[strum(serialize = "playlist-read-collaborative")]
        PlaylistReadCollaborative,

        #[strum(serialize = "user-read-email")]
        UserReadEmail,
        #[strum(serialize = "user-read-birthdate")]
        UserReadBirthDate,
        #[strum(serialize = "user-read-private")]
        UserReadPrivate,

        #[strum(serialize = "user-read-playback-state")]
        UserReadPlaybackState,
        #[strum(serialize = "user-modify-playback-state")]
        UserModifyPlaybackState,
        #[strum(serialize = "user-read-currently-playing")]
        UserReadCurrentlyPlaying,

        #[strum(serialize = "app-remote-control")]
        AppRemoteControl,
        #[strum(serialize = "streaming")]
        Streaming,

        #[strum(serialize = "user-follow-read")]
        UserFollowRead,
        #[strum(serialize = "user-follow-modify")]
        UserFollowModify,
    }
    pub struct Auth {
        /// The Spotify Application Client ID
        pub client_id: String,
        /// Required by the Spotify API.
        pub response_type: String,
        /// The URI to redirect to after the user grants or denies permission.
        pub redirect_uri: Url,
        /// A random generated string that can be useful for correlating requests and responses.
        pub state: String,
        /// Vec of Spotify Scopes.
        #[derive(Into)]
        pub scope: Vec<Scope>,
        /// Whether or not to force the user to approve the app again if they’ve already done so.
        pub show_dialog: bool,
    }

    impl Default for Auth {
        fn default() -> Self {
            // Load local .env file.
            dotenv().ok();

            Auth {
                client_id: match env::var("CLIENT_ID") {
                    Ok(x) => x,
                    Err(_) => "INVALID_ID".to_string(),
                },
                response_type: "code".to_owned(),
                redirect_uri: Url::parse(
                    &env::var("REDIRECT_URI")
                        .unwrap_or_else(|_| "http://localhost:8000/callback".to_string()),
                )
                .unwrap(),
                state: generate_random_string(20),
                scope: vec![],
                show_dialog: false,
            }
        }
    }

    impl Auth {
        pub fn scope_into_string(&self) -> String {
            self.scope
                .iter()
                .map(|x| x.clone().to_string())
                .collect::<Vec<String>>()
                .join(" ")
        }
    }

}
