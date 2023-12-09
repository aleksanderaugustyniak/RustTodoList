use super::*;


fn check_desription(task: &Task, expected_description: String)
{
    assert_eq!(
        task.description,
        expected_description,
        "Expected description: '{}', Actual description: '{}'",
        expected_description,
        task.description
    );
}

#[test]
fn test_new_task() 
{
    let expected_description = String::from("write tests");
    let task = Task::new(String::from("Write tests"));
    check_desription(&task, expected_description);
    assert!(!task.completed, "New task should not be completed");
}

#[test]
fn test_empty_description() 
{
    let expected_description = String::from("");
    let task = Task::new(String::from(""));
    task.show();
    check_desription(&task, expected_description);
    assert!(!task.completed, "New task should not be completed");
}

#[test]
fn test_task_complited()
{
    let mut task = Task::new(String::from("Dummy task"));
    task.complete();
    assert!(task.completed, "Task not comleted after comlete call");
}

#[test]
fn test_task_uncomplited()
{
    let mut task = Task::new(String::from("Dummy task"));
    task.complete();
    task.complete();
    assert!(!task.completed, "Task comleted after even number of comlete method calls");
}
