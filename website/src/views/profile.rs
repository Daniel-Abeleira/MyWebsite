use dioxus::prelude::*;

#[component]
pub fn Profile() -> Element {
    rsx! {
        div {
            h2 { "Profile" }
            div {
                style: "display: flex; gap: 2rem; align-items: flex-start; justify-content: center;",
                div {
                    style: "flex: 1;",
                    h3 { "Introduction" }
                    p { "Hi, I'm Daniel. I started making games with Unity when I was 14, and had made games with Scratch and even handmade card games before then as a kid. Over the years I have been honing my skills in game design and programming to channel my passion for game development into great games." }
                    p { "I specialize in Gameplay Programming, making robust systems for the most complex mechanics of modern games, such as NPC decision-making, simulated economies and histories, inventory and item systems, and generally flagship mechanics for more complex titles." }
                    p { "I am meticulous in my work. To the fullest of my abilities, I make sure any code I touch is maintainable, scalable, easy to debug, readable, and hard to create bugs with. I have a deep understanding of C++ best practices and game programming patterns, and have done extensive research to learn from the ideas and mistakes of well-known developers (as well as my own mistakes)." }
                    p { "I have worked with artists and other programmers as a united, well-oiled machine to bring projects from concept to reality. I am reportedly easy to work with, polite, amicable, and respectful of other people's time and project requirements. I also have a tendency to assume leadership and mentorship roles in teams I join when I can share some of my expertise with colleagues of different backgrounds. I make sure to communicate my progress and concerns clearly and in time, and ask my teammates about theirs to see if I can help." }
                    p { "Time management and personal organization have always been important to me. I use tools like Trello/HacknPlan, Obsidian/Notion and GitHub/Perforce wherever I can. Generally, I am also a pretty self-driven person that can be trusted to find the best way to accomplish a task without much supervision." }
                    p { "During the past few years, I have developed an interest in Rust, (de)serialization (e. g. with JSON) and embedded programming (e. g. with lua), so I am open to introducing those technologies to projects that could benefit from them if the opportunity arises. In the context of games the latter two are extremely beneficial for modding, which in turn has been argued to extend a project's lifespan and relevance immensely." }
                    p { "I was born in Spain in 2004, where I grew up. I spent 9th grade in Switzerland in an IB school and then after high school did 3 years of a Bachelor's Degree in Game Development at Howest DAE in Belgium. I speak Spanish and English fluently and enough French and German to handle living in the relevant countries as long as only English/Spanish is required for work." }
                }
                div {
                    style: "flex: 1;",
                    h3 { "Experience - Main Languages/Engines" }
                    ul {
                        li { "C# & Unity: 8 years" }
                        li { "C++: 4 years" }
                        li { "Rust: 2 years" }
                        li { "Unreal Engine: 2 years" }
                    }
                    h3 { "Experience - Tooling/Embedded Languages" }
                    ul {
                        li { "Lua: 2 years" }
                        li { "JSON: 8 years" }
                        li { "SQL: 2 years" }
                        li { "HTTP and UDP: 2 years" }
                        li { "HTML & CSS: 4 years" }
                    }
                    h3 { "Experience - Organization/Source Control" }
                    ul {
                        li { "Git: 6 years" }
                        li { "Perforce: 2 years" }
                        li { "HacknPlan: 2 years" }
                        li { "Trello: 8 years" }
                    }
                    h3 { "Experience - Essentials" }
                    ul {
                        li { "Google & Microsoft Office suites: >8 years" }
                    }
                    h3 { "Experience - Asset Creation" }
                    ul {
                        li { "Blender: 8 years" }
                        li { "GIMP/Photoshop/Krita/Aseprite: 8 years" }
                        li { "Audacity: 8 years" }
                        li { "OBS and Wondershare Filmora: 8 years" }
                        li { "Maya: 3 years" }
                        li { "Substance Painter & Designer: 4 years (of active use, stopped using them around 4 years ago)" }
                    }
                }
            }
        }
    }
}
