#[macro_export]
macro_rules! delete_resource_where_fields {
    ($resource:ty, $params:expr) => {{
        use crate::database::connection::get_connection;
        use pluralizer::pluralize;
        use anyhow::anyhow;

        async {
            let resource_name = pluralize(&stringify!($resource).to_lowercase(), 2, false);
            let pool = get_connection().await;

            let fields = $params.iter().map(|field| field.0.to_string()).collect::<Vec<String>>();
            let values = $params.iter().map(|field| field.1.to_string()).collect::<Vec<String>>();
            let mut query = format!("DELETE FROM {} WHERE ", resource_name);
            for (i, field) in fields.iter().enumerate() {
                query.push_str(&format!("{} = ${}", field, i + 1));
                if i < fields.len() - 1 {
                    query.push_str(" AND ");
                }
            }

            let mut query = sqlx::query(&query);
            for (_, value) in values.iter().enumerate() {
                query = query.bind(value);
            }

            match query.execute(&pool).await {
                Ok(_) => Ok(()),
                Err(e) => Err(anyhow!(e)),
            }
        }
    }};
}

#[macro_export]
macro_rules! delete_resource {
    ($resource:expr, $id:expr) => {
        delete_resource_where_fields!($resource, vec![("id", $id)]).await
    };
}
