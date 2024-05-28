/*
 * @Author: plucky
 * @Date: 2023-05-24 22:29:37
 * @LastEditTime: 2023-05-24 22:33:24
 * @Description: 
 */

// 外观模式是一种结构性设计模式，它为复杂系统提供了一个简化的接口，使其更易于使用，并减少了其对其他组件的依赖。 
// 外观模式定义了一个更高级别的接口，使复杂系统更易于使用。它为系统提供了一个简化的接口，隐藏了子系统及其相互作用的复杂性。客户端可以使用外观接口执行必要的任务，而无需了解底层子系统的工作原理。

// 子系统1，用于视频剪辑
struct VideoCutter;
 impl VideoCutter {
    fn cut_video(&self) {
        println!("Video is cutting...");
    }
}
 // 子系统2，用于特效添加
struct VideoEffects;
 impl VideoEffects {
    fn add_effects(&self) {
        println!("Effects are adding to the video...");
    }
}
 // 子系统3，用于字幕添加
struct VideoSubtitle;
 impl VideoSubtitle {
    fn add_subtitle(&self) {
        println!("Subtitle is adding to the video...");
    }
}
 // 外观模式，用于封装视频编辑器的复杂子系统
struct VideoEditorFacade {
    cutter: VideoCutter,
    effects: VideoEffects,
    subtitle: VideoSubtitle,
}
 impl VideoEditorFacade {
    fn new() -> VideoEditorFacade {
        VideoEditorFacade {
            cutter: VideoCutter{},
            effects: VideoEffects{},
            subtitle: VideoSubtitle{},
        }
    }
     fn edit_video(&self) {
        self.cutter.cut_video();
        self.effects.add_effects();
        self.subtitle.add_subtitle();
        println!("Video editing is completed!");
    }
}
 // 客户端代码
fn main() {
    let editor = VideoEditorFacade::new();
    editor.edit_video();
}

// 在这个例子中，我们首先定义了三个视频编辑器的子系统： VideoCutter 、 VideoEffects  和  VideoSubtitle 。然后我们定义了一个外观结构  VideoEditorFacade ，它包含了这三个子系统的实例，并实现了一个  edit_video()  方法，该方法按照一定的顺序调用三个子系统的方法。 
 
// 在客户端代码中，我们创建了一个  VideoEditorFacade  的实例并调用了  edit_video()  方法。此时，我们只需要与外观结构交互即可进行视频编辑，而无需了解其复杂的子系统。 