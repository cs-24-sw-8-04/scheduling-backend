use crate::data_model::{event::Event, graph::DescreteGraph, task::Task, time::Timespan};

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

        let greatest = self
            .graph
            .get_values()
            .iter()
            .max_by(|x, y| x.0.cmp(&y.0))
            .unwrap();
        let mut i = 0;

        for task in &self.tasks {
            self.add_event(i, task, greatest);
            i += 1;
        }

        Ok(())
    }

    fn add_event(&self, id: i64, task: &Task, timeslot: &(i32, Timespan)) {
        self.events.push(Event {
            id,
            device_id: task.device_id,
            version_nr: id,
            start_time: timeslot.1.start,
        })
    }
}