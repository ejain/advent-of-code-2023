use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::fs::read_to_string;
use counter::Counter;
use indexmap::IndexMap;

fn main() {
    let input = read_to_string("data/20.txt").unwrap();
    println!("Part 1: {}", solve_part_1(&input));
}

fn solve_part_1(input: &str) -> u64 {
    let mut machine = Machine::parse(input);
    (0..1000).for_each(|_| machine.push());
    machine.num_sent(Pulse::Low) * machine.num_sent(Pulse::High)
}

struct Machine {
    modules: IndexMap<String, Box<dyn Module>>,
    cables: IndexMap<String, Vec<String>>,
    queue: VecDeque<Event>,
    stats: Counter<Pulse>,
}

impl Machine {
    fn parse(input: &str) -> Self {
        let mut machine = Self { modules: IndexMap::new(), cables: IndexMap::new(), queue: VecDeque::new(), stats: Counter::new() };
        for line in input.lines().map(str::trim).filter(|line| !line.is_empty()) {
            let (prefix, postfix) = line.split_once(" -> ").unwrap();
            let (label, module) = Self::parse_module(prefix);
            machine.modules.insert(label.to_string(), module);
            let cables = machine.cables.entry(label.to_string()).or_default();
            for to_label in postfix.split(", ") {
                cables.push(to_label.to_string());
            }
        }
        for to_label in machine.cables.values().flatten() {
            machine.modules.entry(to_label.to_string()).or_insert(Box::new(UntypedModule::new()));
        }
        for (sender, receivers) in &machine.cables {
            for receiver in receivers {
                if let Some(module) = machine.modules.get_mut(receiver) {
                    module.register(sender);
                }
            }
        }
        machine
    }

    fn parse_module(s: &str) -> (String, Box<dyn Module>) {
        if s == BroadcastModule::LABEL {
            (s.to_string(), Box::new(BroadcastModule::new()))
        } else if let Some(label) = s.strip_prefix('%') {
            (label.to_string(), Box::new(FlipFlopModule::new()))
        } else if let Some(label) = s.strip_prefix('&') {
            (label.to_string(), Box::new(ConjunctionModule::new()))
        } else {
            (s.to_string(), Box::new(UntypedModule::new()))
        }
    }

    fn push(&mut self) {
        self.stats[&Pulse::Low] += 1;
        self.process(&Event::new("", BroadcastModule::LABEL, Pulse::Low));
        while let Some(event) = self.queue.pop_front() {
            self.process(&event);
        }
    }

    fn process(&mut self, event: &Event) {
        let module = self.modules.get_mut(&event.receiver).unwrap();
        let pulse = module.process(event);
        if pulse != Pulse::None {
            self.transmit(&event.receiver, pulse);
        }
    }

    fn transmit(&mut self, sender: &str, pulse: Pulse) {
        for receiver in self.cables.get(sender).unwrap_or(&Vec::new()) {
            self.queue.push_back(Event::new(sender, receiver, pulse));
            self.stats[&pulse] += 1;
        }
    }

    fn num_sent(&self, pulse: Pulse) -> u64 {
        self.stats[&pulse] as u64
    }
}

trait Module {
    fn process(&mut self, event: &Event) -> Pulse;

    fn register(&mut self, _source: &str) {}
}

#[derive(Clone, Debug)]
struct BroadcastModule {}

impl BroadcastModule {
    const LABEL: &'static str = "broadcaster";

    fn new() -> Self {
        Self {}
    }
}

impl Module for BroadcastModule {
    fn process(&mut self, _event: &Event) -> Pulse {
        Pulse::Low
    }
}

#[derive(Clone, Debug)]
struct FlipFlopModule {
    state: bool,
}

impl FlipFlopModule {
    fn new() -> Self {
        Self { state: false }
    }
}

impl Module for FlipFlopModule {
    fn process(&mut self, event: &Event) -> Pulse {
        if event.pulse == Pulse::Low {
            if self.state {
                self.state = false;
                Pulse::Low
            } else {
                self.state = true;
                Pulse::High
            }
        } else {
            Pulse::None
        }
    }
}

#[derive(Clone, Debug)]
struct ConjunctionModule {
    state: HashMap<String, Pulse>,
}

impl ConjunctionModule {
    fn new() -> Self {
        Self { state: HashMap::new() }
    }
}

impl Module for ConjunctionModule {
    fn process(&mut self, event: &Event) -> Pulse {
        self.state.insert(event.sender.to_string(), event.pulse);
        if self.state.values().all(|&pulse| pulse == Pulse::High) {
            Pulse::Low
        } else {
            Pulse::High
        }
    }

    fn register(&mut self, source: &str) {
        self.state.insert(source.to_string(), Pulse::Low);
    }
}

#[derive(Clone, Debug)]
struct UntypedModule {}

impl UntypedModule {
    fn new() -> Self {
        Self {}
    }
}

impl Module for UntypedModule {
    fn process(&mut self, _event: &Event) -> Pulse {
        Pulse::None
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Pulse {
    None,
    High,
    Low,
}

#[derive(Clone, Debug)]
struct Event {
    sender: String,
    receiver: String,
    pulse: Pulse,
}

impl Event {
    fn new(sender: &str, receiver: &str, pulse: Pulse) -> Self {
        Self { sender: sender.to_string(), receiver: receiver.to_string(), pulse }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_flip_flop_module() {
        let mut module = FlipFlopModule::new();
        assert_eq!(module.process(&Event::new("", "", Pulse::None)), Pulse::None);
        assert_eq!(module.process(&Event::new("", "", Pulse::High)), Pulse::None);
        assert_eq!(module.process(&Event::new("", "", Pulse::Low)), Pulse::High);
        assert_eq!(module.process(&Event::new("", "", Pulse::None)), Pulse::None);
        assert_eq!(module.process(&Event::new("", "", Pulse::High)), Pulse::None);
        assert_eq!(module.process(&Event::new("", "", Pulse::Low)), Pulse::Low);
    }

    #[test]
    fn test_conjunction_module() {
        let mut module = ConjunctionModule::new();
        assert_eq!(module.process(&Event::new("a", "", Pulse::None)), Pulse::High);
        assert_eq!(module.process(&Event::new("b", "", Pulse::Low)), Pulse::High);
        assert_eq!(module.process(&Event::new("a", "", Pulse::High)), Pulse::High);
        assert_eq!(module.process(&Event::new("b", "", Pulse::High)), Pulse::Low);
        assert_eq!(module.process(&Event::new("a", "", Pulse::Low)), Pulse::High);
    }

    #[test]
    fn test_push_once() {
        let mut machine = Machine::parse("
            broadcaster -> a, b, c
            %a -> b
            %b -> c
            %c -> inv
            &inv -> a
        ");

        machine.push();
        assert_eq!(machine.num_sent(Pulse::Low), 8);
        assert_eq!(machine.num_sent(Pulse::High), 4);
    }

    #[test]
    fn test_push_repeated() {
        let mut machine = Machine::parse("
            broadcaster -> a
            %a -> inv, con
            &inv -> b
            %b -> con
            &con -> output
        ");

        machine.push();
        assert_eq!(machine.num_sent(Pulse::Low), 4);
        assert_eq!(machine.num_sent(Pulse::High), 4);

        machine.push();
        assert_eq!(machine.num_sent(Pulse::Low), 4 + 4);
        assert_eq!(machine.num_sent(Pulse::High), 4 + 2 );

        machine.push();
        assert_eq!(machine.num_sent(Pulse::Low), 4 + 4 + 5);
        assert_eq!(machine.num_sent(Pulse::High), 4 + 2 + 3);

        machine.push();
        assert_eq!(machine.num_sent(Pulse::Low), 4 + 4 + 5 + 4);
        assert_eq!(machine.num_sent(Pulse::High), 4 + 2 + 3 + 2);
    }

        #[test]
        fn test_solve_part_1() {
            assert_eq!(solve_part_1("
                broadcaster -> a, b, c
                %a -> b
                %b -> c
                %c -> inv
                &inv -> a
            "), 8000 * 4000);

            assert_eq!(solve_part_1("
                broadcaster -> a
                %a -> inv, con
                &inv -> b
                %b -> con
                &con -> output
            "), 4250 * 2750);
        }
    }
