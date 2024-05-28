/*
 * @Author: plucky
 * @Date: 2023-11-06 07:50:58
 * @LastEditTime: 2023-11-06 08:20:04
 */

use std::error::Error;

use syrette::ptr::TransientPtr;
use syrette::{injectable, DIContainer};


trait ICat
{
    fn sound(&self);
}

struct Cat {}

#[injectable(ICat)] // 用 ICat 的接口注入 Cat
impl Cat
{
    fn new() -> Self
    {
        Self {}
    }
}

impl ICat for Cat
{
    fn sound(&self)
    {
        println!("cat meow");
    }
}

trait IAnimal
{
    fn fight(&self);
}

struct Animal
{
    cat: TransientPtr<dyn ICat>,
}

#[injectable(IAnimal)] // 用 IAnimal 的接口注入 Animal
impl Animal
{
    fn new(cat: TransientPtr<dyn ICat>) -> Self
    {
        Self { cat }
    }
}

impl IAnimal for Animal
{
    fn fight(&self)
    {
        self.cat.sound();
    }
}

fn main() -> Result<(), Box<dyn Error>>
{
    let mut di_container = DIContainer::new();

    // 创建 ICat 接口与 Cat 类型的绑定
    di_container.bind::<dyn ICat>().to::<Cat>()?;

    // 创建与 IAnimal 接口与 Animal 类型的绑定
    di_container.bind::<dyn IAnimal>().to::<Animal>()?;

    // 创建一个具有所有依赖关系的 IAnimal 自动注入Cat的实例
    let animal = di_container.get::<dyn IAnimal>()?.transient()?;

    animal.fight();

    println!();

    Ok(())
}