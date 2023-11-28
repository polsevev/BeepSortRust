use crate::BarPlugin::Bar;
use async_trait::async_trait;

#[derive(Clone, Debug)]
pub struct GuiVec {
    pub list: Vec<Bar>,
    initialSize: usize,
    pub lastTime: f64,
    pub reads: i32,
    pub writes: i32,
    pub comps: i32,
    isPaused: bool,
    delay: f32,
    pub done: bool,
    renderSkip: i32,
    skipped: i32,
    lastTouched: Vec<usize>,
    lastPlayed: f64,
}
#[async_trait]
pub trait SortingList {
    async fn new(length: usize, delay: f32) -> Self;

    fn len(&self) -> usize;

    async fn swap(&mut self, index1: usize, index2: usize) -> bool;

    async fn draw(&mut self);

    fn randomize(&mut self);

    fn elements(&mut self) -> std::slice::Iter<'_, Bar>;

    fn get(&mut self, i: usize) -> &Bar;

    fn lessThan(&mut self, a: usize, b: usize) -> bool;

    fn lessThanEqual(&mut self, a: usize, b: usize) -> bool;

    fn isSorted(&mut self) -> bool;

    async fn set(&mut self, i: usize, elem: Bar) -> bool;

    async fn show(&mut self);

    fn getListClone(&self) -> Vec<Bar>;
}
#[async_trait]
impl SortingList for GuiVec {
    async fn new(length: usize, delay: f32) -> Self {
        let colorStep = 360. / length as f32;
        let mut list: Vec<Bar> = vec![];
        let freqStep = 50. + ((2000. - 50.) / length as f32);

        for i in 1..length + 1 {
            let frequency = i as f32 * freqStep;
            list.push(Bar::new(i, (colorStep * i as f32) / 360.));
        }

        //Generate sounds
        GuiVec {
            list,
            initialSize: length as usize,
            lastTime: 0.0,
            reads: 0,
            writes: 0,
            comps: 0,
            isPaused: false,
            delay,
            done: false,
            renderSkip: 1,
            skipped: 0,
            lastTouched: Vec::with_capacity(2),
            lastPlayed: 0.,
        }
    }

    async fn draw(&mut self) {}

    fn len(&self) -> usize {
        self.list.len()
    }

    async fn swap(&mut self, index1: usize, index2: usize) -> bool {
        self.writes += 2;
        self.reads += 2;
        self.list.swap(index1, index2);

        self.lastTouched.clear();
        self.lastTouched.push(index1);
        self.lastTouched.push(index2);
        self.draw().await;

        self.done
    }
    fn randomize(&mut self) {}

    fn elements(&mut self) -> std::slice::Iter<'_, Bar> {
        self.list.iter()
    }

    fn get(&mut self, i: usize) -> &Bar {
        self.reads += 1;
        self.lastTouched.clear();
        self.lastTouched.push(i);
        self.list.get(i).unwrap()
    }
    fn lessThan(&mut self, a: usize, b: usize) -> bool {
        self.comps += 1;
        return self.get(a).position < self.get(b).position;
    }
    fn lessThanEqual(&mut self, a: usize, b: usize) -> bool {
        self.comps += 1;
        return self.get(a).position <= b;
    }
    fn isSorted(&mut self) -> bool {
        self.reads += self.len() as i32;
        let mut prev = 0;
        for bar in self.list.iter() {
            if bar.position < prev {
                return false;
            } else {
                prev = bar.position;
            }
        }
        true
    }
    async fn set(&mut self, i: usize, elem: Bar) -> bool {
        self.writes += 1;
        self.reads += 1;
        self.list[i] = elem;
        self.draw().await;

        self.lastTouched.clear();
        self.lastTouched.push(i);
        self.done
    }
    async fn show(&mut self) {
        loop {
            if !self.done {
                self.draw().await
            } else {
                break;
            }
        }
    }

    fn getListClone(&self) -> Vec<Bar> {
        self.list.clone()
    }
}

pub struct NonGuiVec {
    pub list: Vec<Bar>,
}
#[async_trait]
impl SortingList for NonGuiVec {
    async fn new(length: usize, delay: f32) -> Self {
        let mut list = Vec::new();
        for i in 0..(length as usize) {
            list.push(Bar::new(i, i as f32))
        }
        NonGuiVec { list: list }
    }

    fn len(&self) -> usize {
        self.list.len()
    }

    async fn swap(&mut self, index1: usize, index2: usize) -> bool {
        self.list.swap(index1, index2);
        false
    }

    async fn draw(&mut self) {
        self.swap(0, 0).await;
    }

    fn randomize(&mut self) {}

    fn elements(&mut self) -> std::slice::Iter<'_, Bar> {
        self.list.iter()
    }

    fn get(&mut self, i: usize) -> &Bar {
        self.list.get(i).unwrap()
    }
    fn lessThan(&mut self, a: usize, b: usize) -> bool {
        return self.get(a).position < self.get(b).position;
    }
    fn lessThanEqual(&mut self, a: usize, b: usize) -> bool {
        return self.get(a).position <= b;
    }
    fn isSorted(&mut self) -> bool {
        let mut prev = 0;
        for bar in self.list.iter() {
            if bar.position < prev {
                return false;
            } else {
                prev = bar.position;
            }
        }
        true
    }
    async fn set(&mut self, i: usize, elem: Bar) -> bool {
        self.list[i] = elem;
        self.draw().await;

        false
    }
    async fn show(&mut self) {}
    fn getListClone(&self) -> Vec<Bar> {
        self.list.clone()
    }
}
