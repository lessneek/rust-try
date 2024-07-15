use futures::prelude::*;

pub async fn some_task(_input: &InputData) -> Result<OutputData, Error> {
    Ok(OutputData {})
}

pub async fn para() -> Result<(), Error> {
    let input_data = vec![];

    let _results =
        futures::stream::iter(input_data.into_iter().map(|input_data: InputData| {
            tokio::spawn(async move { some_task(&input_data).await })
        }))
        .buffer_unordered(3)
        .try_collect::<Vec<_>>()
        .map_err(|_| Error {})
        .await?;

    // results

    Ok(())
}

pub struct InputData {}

pub struct OutputData {}

pub struct Error {}
