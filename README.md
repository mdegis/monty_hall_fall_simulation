# Monty Hall / Fall Simulation

This repo contains statistical prove (simulation) for Monty Hall / Fall problem  

## Getting Started

Suppose you're on a game show, and you're given the choice of three doors: Behind one door is a car; behind the others, goats. You pick a door, say No. 1, and the host, who knows what's behind the doors, opens another door, say No. 3, which has a goat. He then says to you, "Do you want to pick door No. 2?" Is it to your advantage to switch your choice?

Vos Savant's response was that the contestant should switch to the other door. Under the standard assumptions, contestants who switch have a 2/3 chance of winning the car, while contestants who stick to their initial choice have only a 1/3 chance.

Also, in "Monty Fall" or "Ignorant Monty" version, the host does not know what lies behind the doors, and opens one at random that happens not to reveal the car. Therefore, switching wins the car half of the time.

### Simple solutions

The solution presented by vos Savant in Parade shows the three possible arrangements of one car and two goats behind three doors and the result of staying or switching after initially picking door 1 in each case:

| Behind door 1 | Behind door 2 | Behind door 3 | Result if staying at door #1 | Result if switching to the door offered |
|---------------|---------------|---------------|------------------------------|-----------------------------------------|
| Car           | Goat          | Goat          | Wins car                     | Wins goat                               |
| Goat          | Car           | Goat          | Wins goat                    | Wins car                                |
| Goat          | Goat          | Car           | Wins goat                    | Wins car                                |

A player who stays with the initial choice wins in only one out of three of these equally likely possibilities, while a player who switches wins in two out of three.

An intuitive explanation is that, if the contestant initially picks a goat (2 of 3 doors), the contestant will win the car by switching because the other goat can no longer be picked, whereas if the contestant initially picks the car (1 of 3 doors), the contestant will not win the car by switching (Carlton 2005, concluding remarks). The fact that the host subsequently reveals a goat in one of the unchosen doors changes nothing about the initial probability.


### Run Simulation

Make sure you have successfully installed Rust and Cargo and in root directory run:

```
cargo run
```

Your output will be something like this:

```
     Finished debug [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/monty_hall_fall_simulation`
Running 10000 simulations where the host makes a random guess...
If they pick the car, the universe explodes and we discard the trial
Exploded 3390 times
Switched door 3317 times with 1689 wins and 1628 losses
Kept our choice 3293 times with 1613 wins and 1680 losses
Estimated chance to explode (should be 0.333): 0.339
Estimated chance to win if we switch (should be 0.5): 0.509
Estimated chance to win if we don't (should be 0.5): 0.490
----
Running 10000 simulations where the host precisely avoids the car...
Switched door 4944 times with 3286 wins and 1658 losses
Kept our choice 5056 times with 1682 wins and 3374 losses
Estimated chance to win if we switch (should be 0.666): 0.665
Estimated chance to win if we don't (should be 0.333): 0.333

```

![image](image.png?raw=true "image")


## Built With

* [Rust](https://www.rust-lang.org/en-US/) is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.
* [Cargo](https://crates.io/) - Rust’s package manager

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests to us.

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/mdegis/monty_hall_fall_simulation/tags). 

## Authors

* **Melih Değiş** - *Initial work* - [mdegis](https://github.com/mdegis)

See also the list of [contributors](https://github.com/mdegis/monty_hall_fall_simulation/contributors) who participated in this project.

## License

This project is licensed under the GPLv3 License - see the [LICENSE.md](LICENSE.md) file for details.
