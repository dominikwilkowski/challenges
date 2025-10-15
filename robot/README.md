## The Problem

We have installed a robot in our warehouse and now we need to be able to send commands to control it.
We need you to implement the primary control mechanism.

For convenience the robot moves along a grid in the roof of the warehouse and we've made sure that all of our warehouses are built so that the dimensions of the grid are 10 by 10.
We've also made sure that all our warehouses are aligned along north-south and east-west axes.

All of the commands to the robot consist of a single capital letter and different commands are delineated by whitespace.

### Requirements

* Create a way to send a series of commands to the robot
* Make sure the robot doesn't try to break free and move outside the boundary of the warehouse

The robot should accept the following commands:

* `N` move north
* `W` move west
* `E` move east
* `S` move south

### Example command sequences

The command sequence: `N E S W` will move the robot in a full square, returning it to where it started.

If the robot starts in the south-west corner of the warehouse then the following commands will move it to the middle of the warehouse.

`N E N E N E N E`

## How to run

```rust
fn main() {
	let mut my_wearhouse = Wearhouse::new(Size(10, 10), Position(9, 0));
	println!("{}", my_wearhouse.visualize());
	my_wearhouse.batch_moving("N E N E N E N E");
	println!("{}", my_wearhouse.visualize());
}
```
