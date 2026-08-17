// Import DateTime and Utc from chrono for timestamping task creation
use chrono::{DateTime, Utc};
// Import serde traits for serializing and deserializing tasks to JSON
use serde::{Deserialize, Serialize};
// Import Uuid generator for assigning unique identifiers to each task
use uuid::Uuid;

// Represents an individual user task item with title and pomodoro statistics
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Task {
    // Unique identifier for the task
    pub id: String,
    // Human-readable title/description of the task
    pub title: String,
    // Completion status flag
    pub completed: bool,
    // Number of Pomodoro focus sessions spent working on this task
    pub pomodoros_spent: u32,
    // Estimated number of Pomodoros expected to complete this task
    pub pomodoros_estimated: u32,
    // UTC timestamp when this task was created
    pub created_at: DateTime<Utc>,
}

impl Task {
    // Constructor function to create a new task with an automatic UUID and timestamp
    pub fn new(title: String, estimated: u32) -> Self {
        // Build and return new Task instance
        Self {
            // Generate a random version 4 UUID string
            id: Uuid::new_v4().to_string(),
            // Set the given title
            title,
            // New tasks start as uncompleted
            completed: false,
            // New tasks start with 0 pomodoros spent
            pomodoros_spent: 0,
            // Set user-specified estimated pomodoro count
            pomodoros_estimated: estimated,
            // Record current UTC timestamp
            created_at: Utc::now(),
        }
    }
}

// Filter enumeration for filtering tasks in the task view
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskFilter {
    // Display all tasks regardless of status
    All,
    // Display only active (uncompleted) tasks
    Active,
    // Display only completed tasks
    Completed,
}

// Default filter is to show All tasks
impl Default for TaskFilter {
    // Return All filter as default
    fn default() -> Self {
        // Return TaskFilter::All variant
        TaskFilter::All
    }
}

// Manager structure managing the task list, selections, and active target
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TaskManager {
    // List of all stored tasks
    pub tasks: Vec<Task>,
    // Optional ID of the currently focused task linked to the Pomodoro timer
    pub active_task_id: Option<String>,
    // Transient index of currently selected row in UI (not saved to disk)
    #[serde(skip)]
    pub selected_index: usize,
    // Transient active filter for viewing tasks (not saved to disk)
    #[serde(skip)]
    pub filter: TaskFilter,
}

impl TaskManager {
    // Constructor for creating an empty TaskManager
    pub fn new() -> Self {
        // Initialize empty task list and default selection
        Self {
            // Empty task vector
            tasks: Vec::new(),
            // No active target initially
            active_task_id: None,
            // First item selected by default
            selected_index: 0,
            // Show all tasks by default
            filter: TaskFilter::All,
        }
    }

    // Returns vector of indices into self.tasks that match the current filter
    pub fn filtered_indices(&self) -> Vec<usize> {
        // Iterate through tasks with their index
        self.tasks
            .iter()
            .enumerate()
            // Apply filter condition
            .filter(|(_, task)| match self.filter {
                // All: include all tasks
                TaskFilter::All => true,
                // Active: include only uncompleted tasks
                TaskFilter::Active => !task.completed,
                // Completed: include only completed tasks
                TaskFilter::Completed => task.completed,
            })
            // Extract the original index
            .map(|(idx, _)| idx)
            // Collect into a vector
            .collect()
    }

    // Adds a new task with given title and estimated pomodoros
    pub fn add(&mut self, title: String, estimated: u32) {
        // Verify title is not empty after trimming whitespace
        if !title.trim().is_empty() {
            // Create new Task instance
            let task = Task::new(title.trim().to_string(), estimated);
            // If this is the only task, automatically make it the active timer target
            if self.tasks.is_empty() {
                // Set active target ID
                self.active_task_id = Some(task.id.clone());
            }
            // Push task to the task vector
            self.tasks.push(task);
            // Update selected index to highlight the newly created task
            self.selected_index = self.tasks.len().saturating_sub(1);
        }
    }

    // Deletes the currently selected task in the UI list
    pub fn remove_selected(&mut self) {
        // Retrieve indices that match current filter
        let indices = self.filtered_indices();
        // Check if current selected index points to a valid task
        if let Some(&real_idx) = indices.get(self.selected_index) {
            // Remove task from vector
            let removed = self.tasks.remove(real_idx);
            // If the deleted task was the active timer target, reassign target to next incomplete task
            if self.active_task_id.as_deref() == Some(&removed.id) {
                // Find first uncompleted task
                self.active_task_id = self.tasks.iter().find(|t| !t.completed).map(|t| t.id.clone());
            }
            // Get new length of filtered tasks
            let new_indices_len = self.filtered_indices().len();
            // Adjust selected index if it is now out of bounds
            if self.selected_index >= new_indices_len && new_indices_len > 0 {
                // Move selection to last item
                self.selected_index = new_indices_len - 1;
            }
        }
    }

    // Toggles the completed status of the currently selected task
    pub fn toggle_selected(&mut self) {
        // Retrieve filtered indices
        let indices = self.filtered_indices();
        // Get actual index in task vector
        if let Some(&real_idx) = indices.get(self.selected_index) {
            // Borrow task mutably
            if let Some(task) = self.tasks.get_mut(real_idx) {
                // Invert completed boolean
                task.completed = !task.completed;
                // If newly marked completed was active timer target, switch active target
                if task.completed && self.active_task_id.as_deref() == Some(&task.id) {
                    // Reassign to another incomplete task if available
                    self.active_task_id = self.tasks.iter().find(|t| !t.completed).map(|t| t.id.clone());
                }
            }
        }
    }

    // Sets the currently selected task as the active focus target for the timer
    pub fn set_selected_active(&mut self) {
        // Retrieve filtered indices
        let indices = self.filtered_indices();
        // Get actual index in task vector
        if let Some(&real_idx) = indices.get(self.selected_index) {
            // Retrieve task reference
            if let Some(task) = self.tasks.get(real_idx) {
                // Set active target ID
                self.active_task_id = Some(task.id.clone());
            }
        }
    }

    // Increments the pomodoros_spent counter for the active task upon completing a work session
    pub fn increment_active_spent(&mut self) {
        // Check if an active task ID is configured
        if let Some(ref active_id) = self.active_task_id {
            // Find task matching active ID and increment spent count
            if let Some(task) = self.tasks.iter_mut().find(|t| &t.id == active_id) {
                // Add 1 to completed pomodoro count
                task.pomodoros_spent += 1;
            }
        }
    }

    // Returns a reference to the active task struct if one exists
    pub fn active_task(&self) -> Option<&Task> {
        // Look up active task ID
        self.active_task_id
            .as_ref()
            // Find task matching the ID
            .and_then(|id| self.tasks.iter().find(|t| &t.id == id))
    }

    // Moves selection down to the next task in the UI list
    pub fn next(&mut self) {
        // Get count of visible tasks
        let count = self.filtered_indices().len();
        // Check if list is non-empty
        if count > 0 {
            // If not at the end, advance by 1
            if self.selected_index + 1 < count {
                // Increment index
                self.selected_index += 1;
            } else {
                // Wrap around to top
                self.selected_index = 0;
            }
        }
    }

    // Moves selection up to the previous task in the UI list
    pub fn previous(&mut self) {
        // Get count of visible tasks
        let count = self.filtered_indices().len();
        // Check if list is non-empty
        if count > 0 {
            // If not at the beginning, decrement by 1
            if self.selected_index > 0 {
                // Decrement index
                self.selected_index -= 1;
            } else {
                // Wrap around to bottom
                self.selected_index = count - 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // Import super module items
    use super::*;

    // Test adding and completing tasks
    #[test]
    fn test_task_lifecycle() {
        // Initialize task manager
        let mut manager = TaskManager::new();
        // Add task 1
        manager.add("Write unit tests".to_string(), 3);
        // Verify task added
        assert_eq!(manager.tasks.len(), 1);
        // Verify title
        assert_eq!(manager.tasks[0].title, "Write unit tests");
        // Verify estimated pomodoros
        assert_eq!(manager.tasks[0].pomodoros_estimated, 3);
        // Verify it was set as active target automatically
        assert_eq!(manager.active_task().map(|t| t.title.as_str()), Some("Write unit tests"));

        // Increment active spent pomodoro
        manager.increment_active_spent();
        // Verify spent is 1
        assert_eq!(manager.tasks[0].pomodoros_spent, 1);

        // Toggle task completion
        manager.toggle_selected();
        // Verify task completed
        assert!(manager.tasks[0].completed);
    }

    // Test task filtering
    #[test]
    fn test_task_filtering() {
        // Initialize task manager
        let mut manager = TaskManager::new();
        // Add task 1
        manager.add("Task 1".to_string(), 1);
        // Add task 2
        manager.add("Task 2".to_string(), 2);
        // Mark task 1 done
        manager.selected_index = 0;
        manager.toggle_selected();

        // Filter: All (length 2)
        manager.filter = TaskFilter::All;
        assert_eq!(manager.filtered_indices().len(), 2);

        // Filter: Active (length 1)
        manager.filter = TaskFilter::Active;
        assert_eq!(manager.filtered_indices().len(), 1);

        // Filter: Completed (length 1)
        manager.filter = TaskFilter::Completed;
        assert_eq!(manager.filtered_indices().len(), 1);
    }

    #[test]
    fn test_empty_and_whitespace_title_rejected() {
        let mut manager = TaskManager::new();
        manager.add("".to_string(), 1);
        manager.add("    ".to_string(), 2);
        assert_eq!(manager.tasks.len(), 0);
        assert_eq!(manager.active_task_id, None);
    }

    #[test]
    fn test_remove_selected_and_active_reassignment() {
        let mut manager = TaskManager::new();
        manager.add("Task 1".to_string(), 1);
        manager.add("Task 2".to_string(), 2);
        manager.add("Task 3".to_string(), 3);

        // Task 1 is active
        assert_eq!(manager.active_task_id, Some(manager.tasks[0].id.clone()));

        // Delete selected (Task 3 at index 2 initially)
        manager.selected_index = 2;
        manager.remove_selected();
        assert_eq!(manager.tasks.len(), 2);
        assert_eq!(manager.selected_index, 1);

        // Now select Task 1 and delete it (it's the active task)
        manager.selected_index = 0;
        manager.remove_selected();
        assert_eq!(manager.tasks.len(), 1);
        // Active task must now be Task 2
        assert_eq!(manager.active_task_id, Some(manager.tasks[0].id.clone()));
        assert_eq!(manager.tasks[0].title, "Task 2");

        // Delete last remaining task
        manager.selected_index = 0;
        manager.remove_selected();
        assert_eq!(manager.tasks.len(), 0);
        assert_eq!(manager.active_task_id, None);

        // Calling remove_selected on empty list does not panic
        manager.remove_selected();
        assert_eq!(manager.tasks.len(), 0);
    }

    #[test]
    fn test_toggle_selected_active_task_reassignment() {
        let mut manager = TaskManager::new();
        manager.add("Task 1".to_string(), 1);
        manager.add("Task 2".to_string(), 2);

        // Initially Task 1 is active
        assert_eq!(manager.active_task().unwrap().title, "Task 1");

        // Mark Task 1 as completed
        manager.selected_index = 0;
        manager.toggle_selected();
        assert!(manager.tasks[0].completed);

        // Active task automatically switches to Task 2
        assert_eq!(manager.active_task().unwrap().title, "Task 2");

        // Mark Task 2 as completed
        manager.selected_index = 1;
        manager.toggle_selected();
        assert!(manager.tasks[1].completed);

        // No more incomplete tasks, active_task_id becomes None
        assert_eq!(manager.active_task_id, None);

        // Uncomplete Task 1
        manager.selected_index = 0;
        manager.toggle_selected();
        assert!(!manager.tasks[0].completed);
    }

    #[test]
    fn test_set_selected_active() {
        let mut manager = TaskManager::new();
        manager.add("Task 1".to_string(), 1);
        manager.add("Task 2".to_string(), 2);

        // Select Task 2
        manager.selected_index = 1;
        manager.set_selected_active();
        assert_eq!(manager.active_task().unwrap().title, "Task 2");
    }

    #[test]
    fn test_navigation_next_previous_wrapping() {
        let mut manager = TaskManager::new();
        // Empty list navigation does nothing
        manager.next();
        assert_eq!(manager.selected_index, 0);
        manager.previous();
        assert_eq!(manager.selected_index, 0);

        manager.add("Task 1".to_string(), 1);
        manager.add("Task 2".to_string(), 2);
        manager.add("Task 3".to_string(), 3);

        manager.selected_index = 0;
        manager.next();
        assert_eq!(manager.selected_index, 1);
        manager.next();
        assert_eq!(manager.selected_index, 2);
        // Wraps to 0
        manager.next();
        assert_eq!(manager.selected_index, 0);

        // Wraps backwards to 2
        manager.previous();
        assert_eq!(manager.selected_index, 2);
        manager.previous();
        assert_eq!(manager.selected_index, 1);
    }

    #[test]
    fn test_increment_active_spent_no_active_task() {
        let mut manager = TaskManager::new();
        manager.increment_active_spent(); // should not panic
        assert_eq!(manager.tasks.len(), 0);
    }
}


