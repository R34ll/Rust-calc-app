extern crate meval;

use eframe::{
    App,
    run_native,
    NativeOptions,
    egui, 
};


struct Calculator{
    x:String
}


impl Calculator {
    fn new(ctx:&eframe::CreationContext<'_>)-> Calculator{

        Calculator { x: "".to_string()}
    }
    
}


fn ui_buttons(ui:&mut egui::Ui, text:&str)->egui::Response{


    let button = egui::Button::new(text);

    let button = ui.add_sized(egui::Vec2{
        x:40.0,
        y:40.0
    },button);

    button

}





fn calc(calc:&String) -> f64{
    if calc.chars().count()<=0{return 0.0;}


    return meval::eval_str(calc).unwrap();
}


impl App for Calculator{
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        
        egui::TopBottomPanel::top("Numbers").show(ctx, |ui|{

            ui.add_space(5.0);
            ui.text_edit_singleline(&mut self.x.to_string());
            ui.add_space(5.0);


        });

        egui::CentralPanel::default().show(ctx,|ui|{


            ui.horizontal(|ui|{
                if ui_buttons(ui,"9").clicked(){
                    self.x.push_str("9");
                };

                if ui_buttons(ui,"8").clicked(){
                    self.x.push_str("8");

                };

                if ui_buttons(ui,"7").clicked(){
                    self.x.push_str("7");

                };

                if ui_buttons(ui," < ").clicked(){
                    self.x.pop();

                };
            });


            ui.horizontal(|ui|{
                if ui_buttons(ui,"6").clicked(){
                    self.x.push_str("6");

                };
                if ui_buttons(ui,"5").clicked(){
                    self.x.push_str("5");

                };
                if ui_buttons(ui,"4").clicked(){
                    self.x.push_str("4");

                };
                if ui_buttons(ui,"+").clicked(){
                    self.x.push_str(" + ");

                };
            });

            
            ui.horizontal(|ui|{
                if ui_buttons(ui,"3").clicked(){
                    self.x.push_str("3");

                };
                if ui_buttons(ui,"2").clicked(){
                    self.x.push_str("2");

                };
                if ui_buttons(ui,"1 ").clicked(){
                    self.x.push_str("1");

                };
                if ui_buttons(ui,"-").clicked(){
                    self.x.push_str(" - ");

                };
            });


            ui.horizontal(|ui|{
                let zero_btn = egui::Button::new("0");
                let zero_btn = ui.add_sized(egui::Vec2{
                    x:89.0,
                    y:40.0
                },zero_btn);

                if zero_btn.clicked(){
                    self.x.push_str("0");
                }



                //
                if ui_buttons(ui," / ").clicked(){
                    self.x.push_str(" / ");
                };


                if ui_buttons(ui," = ").clicked(){
                    let res = calc(&self.x);

                    self.x = res.to_string();


                };
            })
        });
    }
}


fn main(){

    let mut wind_conf = NativeOptions::default();
    wind_conf.initial_window_size = Some(egui::Vec2::new(200.0,250.0));
    wind_conf.resizable = false;


    run_native(
        "Calculator",
        wind_conf,
        Box::new(|_cc|Box::new(Calculator::new(_cc)))
    );


}






