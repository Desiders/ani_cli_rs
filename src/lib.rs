pub mod sources {
    pub mod ru {
        pub mod anilibria {
            mod schemas {
                pub mod anime;
                pub mod names;
                pub mod player;
                pub mod playlist;
                pub mod series;
            }
            pub mod api;
            pub mod methods;
            pub mod parser;
            pub mod source;
        }
    }
    pub mod common {
        pub mod methods;
    }
}
pub mod dialog;
pub mod prompt;
