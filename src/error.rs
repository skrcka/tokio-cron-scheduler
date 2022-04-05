#[derive(Debug, Clone, Copy)]
pub enum JobSchedulerError {
    CantRemove,
    CantAdd,
    CantInit,
    TickError,
    CantGetTimeUntil,
    Shutdown,
    ShutdownNotifier,
    AddShutdownNotifier,
    RemoveShutdownNotifier,
    FetchJob,
    SaveJob,
    StartScheduler,
    ErrorLoadingGuidList,
    ErrorLoadingJob,
    CouldNotGetTimeUntilNextTick,
    GetJobData,
    GetJobStore,
    JobTick,
    UpdateJobData,
    NoNextTick,
    CantListGuids,
    CantListNextTicks,
    NotifyOnStateError,
}
