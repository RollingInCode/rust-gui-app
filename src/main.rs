use eframe::{epi::App, run_native};

struct Headlines {
    articles: Vec<NewsCardData>
}

impl Headlines {
    fn new() -> Headlines {
        let iter ={
            title: format!("title{}", a),
            desc: format!("desc{}", a),
            
        }
    } 
}

struct NewsCardData {
    title: String,
    desc: String,
    url: String
}

impl App for Headlines {
    fn update(&mut self, ctx: &eframe::egui::CtxRef, frame: &mut eframe::epi::Frame<'_>) {
        CentralPanel::default().show(ctx, add_contents: |ui: &mut Ui|) {
            ui.label("article text");
        });
    }

    fn name(&self) -> &str {
        "Headlines"
    }
}

fn main() {
    let app: Headlines = Headlines;
    let win_option = NativeOptions::default();

    run_native(app, native options);

}
