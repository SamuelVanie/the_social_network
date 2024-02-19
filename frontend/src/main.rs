slint::slint!{


    import { TextEdit, ScrollView } from "std-widgets.slint";
export component ConvoList inherits Rectangle{
        x: 0;
        background: rgb(50,0,0);
    }

    export component Rooms inherits Rectangle {
        background: rgb(25,25,25);
        width: 100px;
    }

    export component Message inherits Rectangle {
        background: area.has-hover ? rgb(25,25,25): rgb(0,0,0);
        area := TouchArea {
            VerticalLayout {
                author:= TextInput {
                    read-only: true;
                    x:0px;
                    font-size: 14px;
                    font-weight: 700;
                    color: red;
                    text: "User1";
                }
                content:= TextInput {
                    
                    read-only: true;
                    x:0px;
                    font-size: 14px;
                    color: white;
                    text: "Hello  World!";
                }
            }
        }
        
        
        
    }

    export component Chat inherits Rectangle {
        background: rgb(0,0,0);
        VerticalLayout {
            max-width: parent.width;
            spacing: 10px;
            chat:=ScrollView{
                VerticalLayout{
                    spacing: 10px;
                    alignment: end;
                    msg1 := Message {
                        max-width: root.width;
                    }
                    msg2 := Message {
                        max-width: root.width;
                    }
                }
            }
            fill := Rectangle{
                background: rgb(20,20,20);
                padding: 25px;
                height: 30px;
                TextInput {
                    color: white;
                    width: parent.width;
                    height: parent.height;
                }
            }

        }
    }

    export component Members inherits Rectangle {
        background: rgb(50,0,0);
        width: 100px;
    }

    export component ServerView inherits Rectangle{
        HorizontalLayout {
            Rooms {
                
            }
            Chat {

            }
            Members {

            }
        }
    }

    export component mainView {
        height: 1000px;
        width: 1000px;
        HorizontalLayout {
            conv := ConvoList {
                height: 100%;
                width: 70px;
            }
            main := ServerView {
                background: green;
            }
        }
    }
}
fn main() {
    mainView::new().unwrap().run().unwrap();
}
