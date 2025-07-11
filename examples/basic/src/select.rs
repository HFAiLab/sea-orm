use super::*;
use sea_orm::{DbConn, FromQueryResult, entity::*, error::*, query::*};

pub async fn all_about_select(db: &DbConn) -> Result<(), DbErr> {
    find_all(db).await?;

    println!("===== =====\n");

    find_one(db).await?;

    println!("===== =====\n");

    find_one_to_one(db).await?;

    println!("===== =====\n");

    find_one_to_many(db).await?;

    println!("===== =====\n");

    count_fruits_by_cake(db).await?;

    println!("===== =====\n");

    find_many_to_many(db).await?;

    if false {
        println!("===== =====\n");

        all_about_select_json(db).await?;
    }

    println!("===== =====\n");

    find_all_stream(db).await.unwrap();

    println!("===== =====\n");

    find_first_page(db).await.unwrap();

    println!("===== =====\n");

    find_num_pages(db).await.unwrap();

    Ok(())
}

async fn find_all(db: &DbConn) -> Result<(), DbErr> {
    print!("find all cakes: ");

    let cakes: Vec<cake::Model> = Cake::find().all(db).await?;

    println!();
    for cc in cakes.iter() {
        println!("{cc:?}\n");
    }

    print!("find all fruits: ");

    let fruits = Fruit::find().all(db).await?;

    println!();
    for ff in fruits.iter() {
        println!("{ff:?}\n");
    }

    Ok(())
}

async fn find_one_to_one(db: &DbConn) -> Result<(), DbErr> {
    print!("find fruits and cakes: ");

    let fruits_and_cakes: Vec<(fruit::Model, Option<cake::Model>)> =
        Fruit::find().find_also_related(Cake).all(db).await?;

    println!("with loader: ");
    let fruits: Vec<fruit::Model> = Fruit::find().all(db).await?;
    let cakes: Vec<Option<cake::Model>> = fruits.load_one(Cake, db).await?;

    println!();
    for (left, right) in fruits_and_cakes
        .into_iter()
        .zip(fruits.into_iter().zip(cakes.into_iter()))
    {
        println!("{left:?}");
        assert_eq!(left, right);
    }

    Ok(())
}

async fn find_one_to_many(db: &DbConn) -> Result<(), DbErr> {
    print!("find cakes with fruits: ");

    let cakes_with_fruits: Vec<(cake::Model, Vec<fruit::Model>)> = Cake::find()
        .find_with_related(fruit::Entity)
        .all(db)
        .await?;

    println!("with loader: ");
    let cakes: Vec<cake::Model> = Cake::find().all(db).await?;
    let fruits: Vec<Vec<fruit::Model>> = cakes.load_many(Fruit, db).await?;

    println!();
    for (left, right) in cakes_with_fruits
        .into_iter()
        .zip(cakes.into_iter().zip(fruits.into_iter()))
    {
        println!("{left:?}\n");
        assert_eq!(left, right);
    }

    Ok(())
}

impl Cake {
    fn find_by_name(name: &str) -> Select<Self> {
        Self::find().filter(cake::Column::Name.contains(name))
    }
}

async fn find_one(db: &DbConn) -> Result<(), DbErr> {
    print!("find one by primary key: ");

    let cheese: Option<cake::Model> = Cake::find_by_id(1).one(db).await?;
    let cheese = cheese.unwrap();

    println!();
    println!("{cheese:?}");
    println!();

    print!("find one by name: ");

    let chocolate = Cake::find_by_name("chocolate").one(db).await?;

    println!();
    println!("{chocolate:?}");
    println!();

    print!("find models belong to: ");

    let fruits = cheese.find_related(Fruit).all(db).await?;

    println!();
    for ff in fruits.iter() {
        println!("{ff:?}\n");
    }

    Ok(())
}

async fn count_fruits_by_cake(db: &DbConn) -> Result<(), DbErr> {
    #[derive(Debug, FromQueryResult)]
    struct SelectResult {
        name: String,
        num_of_fruits: i32,
    }

    print!("count fruits by cake: ");

    let select = Cake::find()
        .left_join(Fruit)
        .select_only()
        .column(cake::Column::Name)
        .column_as(fruit::Column::Id.count(), "num_of_fruits")
        .group_by(cake::Column::Name);

    let results = select.into_model::<SelectResult>().all(db).await?;

    println!();
    for rr in results.iter() {
        println!("{rr:?}\n");
    }

    Ok(())
}

async fn find_many_to_many(db: &DbConn) -> Result<(), DbErr> {
    print!("find cakes and fillings: ");

    let cakes_with_fillings: Vec<(cake::Model, Vec<filling::Model>)> =
        Cake::find().find_with_related(Filling).all(db).await?;

    println!("with loader: ");
    let cakes: Vec<cake::Model> = Cake::find().all(db).await?;
    let fillings: Vec<Vec<filling::Model>> =
        cakes.load_many_to_many(Filling, CakeFilling, db).await?;

    println!();
    for (left, right) in cakes_with_fillings
        .into_iter()
        .zip(cakes.into_iter().zip(fillings.into_iter()))
    {
        println!("{left:?}\n");
        assert_eq!(left, right);
    }

    print!("find fillings for cheese cake: ");

    let cheese = Cake::find_by_id(1).one(db).await?;

    if let Some(cheese) = cheese {
        let fillings: Vec<filling::Model> = cheese.find_related(Filling).all(db).await?;

        println!();
        for ff in fillings.iter() {
            println!("{ff:?}\n");
        }
    }

    print!("find cakes for lemon: ");

    let lemon = Filling::find_by_id(2).one(db).await?;

    if let Some(lemon) = lemon {
        let cakes: Vec<cake::Model> = lemon.find_related(Cake).all(db).await?;

        println!();
        for cc in cakes.iter() {
            println!("{cc:?}\n");
        }
    }

    Ok(())
}

async fn all_about_select_json(db: &DbConn) -> Result<(), DbErr> {
    find_all_json(db).await?;

    println!("===== =====\n");

    find_together_json(db).await?;

    println!("===== =====\n");

    count_fruits_by_cake_json(db).await?;

    Ok(())
}

async fn find_all_json(db: &DbConn) -> Result<(), DbErr> {
    print!("find all cakes: ");

    let cakes = Cake::find().into_json().all(db).await?;

    println!("\n{}\n", serde_json::to_string_pretty(&cakes).unwrap());

    print!("find all fruits: ");

    let fruits = Fruit::find().into_json().all(db).await?;

    println!("\n{}\n", serde_json::to_string_pretty(&fruits).unwrap());

    Ok(())
}

async fn find_together_json(db: &DbConn) -> Result<(), DbErr> {
    print!("find cakes and fruits: ");

    let cakes_fruits = Cake::find()
        .find_with_related(Fruit)
        .into_json()
        .all(db)
        .await?;

    println!(
        "\n{}\n",
        serde_json::to_string_pretty(&cakes_fruits).unwrap()
    );

    Ok(())
}

async fn count_fruits_by_cake_json(db: &DbConn) -> Result<(), DbErr> {
    print!("count fruits by cake: ");

    let count = Cake::find()
        .left_join(Fruit)
        .select_only()
        .column(cake::Column::Name)
        .column_as(fruit::Column::Id.count(), "num_of_fruits")
        .group_by(cake::Column::Name)
        .into_json()
        .all(db)
        .await?;

    println!("\n{}\n", serde_json::to_string_pretty(&count).unwrap());

    Ok(())
}

async fn find_all_stream(db: &DbConn) -> Result<(), DbErr> {
    use futures_util::TryStreamExt;
    use std::time::Duration;
    use tokio::time::sleep;

    println!("find all cakes paginated: ");
    let mut cake_paginator = cake::Entity::find().paginate(db, 3);
    while let Some(cake_res) = cake_paginator.fetch_and_next().await? {
        for cake in cake_res {
            println!("{cake:?}");
        }
    }

    println!();
    println!("find all fruits paginated: ");
    let mut fruit_paginator = fruit::Entity::find().paginate(db, 3);
    while let Some(fruit_res) = fruit_paginator.fetch_and_next().await? {
        for fruit in fruit_res {
            println!("{fruit:?}");
        }
    }

    println!();
    println!("find all fruits with stream: ");
    let mut fruit_stream = fruit::Entity::find().paginate(db, 3).into_stream();
    while let Some(fruits) = fruit_stream.try_next().await? {
        for fruit in fruits {
            println!("{fruit:?}");
        }
        sleep(Duration::from_millis(250)).await;
    }

    println!();
    println!("find all fruits in json with stream: ");
    let mut json_stream = fruit::Entity::find()
        .into_json()
        .paginate(db, 3)
        .into_stream();
    while let Some(jsons) = json_stream.try_next().await? {
        for json in jsons {
            println!("{json:?}");
        }
        sleep(Duration::from_millis(250)).await;
    }

    Ok(())
}

async fn find_first_page(db: &DbConn) -> Result<(), DbErr> {
    println!("fruits first page: ");
    let page = fruit::Entity::find().paginate(db, 3).fetch_page(0).await?;
    for fruit in page {
        println!("{fruit:?}");
    }

    Ok(())
}

async fn find_num_pages(db: &DbConn) -> Result<(), DbErr> {
    println!("fruits number of page: ");
    let num_pages = fruit::Entity::find().paginate(db, 3).num_pages().await?;
    println!("{num_pages:?}");

    Ok(())
}
