/*
 * @Author: plucky
 * @Date: 2023-05-26 01:51:30
 * @LastEditTime: 2023-05-26 01:53:44
 * @Description: 
 */
// 模版方法模式

// 抽象类  Game 
trait Game {
    // 抽象方法
    fn initialize(&self);
    fn start_play(&self);
    fn end_play(&self);
    // 模板方法
     fn play(&self) {
        self.initialize();
        self.start_play();
        self.end_play();
    }
}
 // 具体类1
struct Cricket;
impl Game for Cricket {
    fn initialize(&self) {
        println!("Cricket Game Initialized! Start playing.");
    }
     fn start_play(&self) {
        println!("Cricket Game Started. Enjoy the game!");
    }
     fn end_play(&self) {
        println!("Cricket Game Finished!");
    }
}
 // 具体类2
struct Football;
impl Game for Football {
    fn initialize(&self) {
        println!("Football Game Initialized! Start playing.");
    }
     fn start_play(&self) {
        println!("Football Game Started. Enjoy the game!");
    }
     fn end_play(&self) {
        println!("Football Game Finished!");
    }
}
 // 客户端代码
fn main() {
    let game: Cricket = Cricket;
    game.play();
     let game: Football = Football;
    game.play();
}

// 模板方法模式（Template Method Pattern）是一种行为设计模式。它定义了一个算法的骨架，将一些步骤的具体实现留给子类来完成。模板方法使得子类可以在不改变算法结构的情况下重新定义算法中的某些步骤。 
// 模板方法模式有以下几个角色组成： 
// 1. 抽象类（Abstract Class）：定义模板方法的骨架，包含抽象方法、具体方法和钩子（Hook）方法。抽象方法由子类实现，具体方法由抽象类或子类实现，钩子方法通常为空或提供默认实现，子类可以选择性地覆盖它们。 
// 2. 具体类（Concrete Class）：实现抽象类中的抽象方法和钩子方法。 
// 模板方法模式的优点在于它提供了一个可以重用的算法骨架，使得算法的实现更加灵活。子类可以在不改变算法结构的情况下重新定义算法中的某些步骤，从而实现个性化定制。同时，它还可以避免代码重复和冗余，提高代码的可维护性

// 在这个例子中，我们创建了一个抽象类  Game ，它定义了一个模板方法  play()  和三个抽象方法  initialize() 、 start_play()  和  end_play() 。然后，我们创建了两个具体类  Cricket  和  Football ，它们实现了  Game  中的抽象方法。最后，我们在客户端代码中创建了  Cricket  和  Football  的实例，并调用它们的  play()  方法。