#[macro_export]
macro_rules! update_resource {
    ($resource:ty, $id:expr, $params:expr) => {{
        use crate::database::connection::get_connection;
        use pluralizer::pluralize;
        use time::OffsetDateTime;

        async {
            let updated_at = OffsetDateTime::now_utc();
            let updated_at_str = updated_at.to_string();

            let resource_name = pluralize(&stringify!($resource).to_lowercase(), 2, false);
            let pool = get_connection().await;

            let mut params = $params.clone();
            params.push(("updated_at", &updated_at_str));

            let fields = params.iter().map(|field| field.0.to_string()).collect::<Vec<String>>();
            let values = params.iter().map(|field| field.1.to_string()).collect::<Vec<String>>();

            let mut query = format!("UPDATE {} SET ", resource_name);
            for (i, field) in fields.iter().enumerate() {
                query.push_str(&format!("{} = ${}", field, i + 1));
                if i < fields.len() - 1 {
                    query.push_str(", ");
                }
            }
            query.push_str(&format!(" WHERE id = ${}", fields.len() + 1));

            let mut query = sqlx::query(&query);
            for (_, value) in values.iter().enumerate() {
                query = query.bind(value);
            }
            query = query.bind($id);

            match query.execute(&pool).await {
                Ok(_) => (),
                Err(e) => return Err(e),
            };

            match find_one_resource_where_fields!($resource, vec![("id", &$id)]).await {
                Ok(resource) => Ok(resource),
                Err(e) => Err(e),
            }
        }
    }};
}
