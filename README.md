# Work in progress!

# A visualization of sorting!

This is a clone of the project [The Sound of Sorting](https://panthema.net/2013/sound-of-sorting/) without the sound (for now)

It is written using the tiny game engine [MacroQuad](https://macroquad.rs/)

WebAssembly version can be found here [beepsort.polsevev.dev](https://beepsort.polsevev.dev)

## TODO:
- Implement sound using browser API
- Clean up user interface, perhaps move to JS?
- Add more visualization modes (circle, points etc)
- Add more algorithms

## Contributing

If you wish to contribute an algorithm to this project, i encourage it!

Algorithms are located in [Algorithm.rs](./src/Algorithm.rs) and are made as follows!

1. Create a case in the match located in the function run
2. Implement the algorithm as an async function
3. Whenever you wish to swap two elements, do so in the following manner:
    - ```if list.swap(a,b).await {return}```
    - This ensures we can exit the run of the algorithm if the user clicks "Exit"
4. Add a button on the front page (will move to a menu in the future!)
5. Success!!!!!