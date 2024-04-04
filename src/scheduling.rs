use crate::data_model::{
    event::Event, 
    graph::DescreteGraph, 
    task::Task, 
    time::DateTimeUtc
};

pub struct Scheduler {
    tasks: Vec<Task>,
    events: Vec<Event>,
    graph: DescreteGraph,
}

impl Scheduler {
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn get_events(&self) -> &Vec<Event> {
        &self.events
    }

    pub fn schedule_naive(&self) -> Result<(), &str> {
        if self.tasks.is_empty() {
            return Err("No tasks to schedule");
        }

        let mut i = 0; 
        let mut greatest = 0;
        let graph_values = self.graph.get_values();

        for value in graph_values {
            if value.0 > greatest {
                greatest = i;
            }
            i += 1;
        }

        self.tasks.iter().for_each(|task| {
            Self::add_event(task.device.id, task.timespan.start)
        });

        Ok(())
    }

    fn add_event(device_id: i64, start_time: DateTimeUtc) {

    }
}