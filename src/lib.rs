use mediawiki::api::Api;
use serde_json::value::Value;

const API_URL: &str = "https://lol.fandom.com/api.php";

pub async fn get_api() -> Result<Api, Box<dyn std::error::Error>> {
    let api = Api::new(API_URL).await.unwrap();

    Ok(api)
}

async fn get_json_response(
    api: &Api,
    params: &[(&str, &str)],
) -> Result<Option<Vec<Value>>, Box<dyn std::error::Error>> {
    let params = api.params_into(params);

    let res = api.get_query_api_json_all(&params).await.unwrap();

    let rows = res["cargoquery"].as_array().unwrap();

    match rows.len() {
        0 => Ok(None),
        _ => {
            let rows: Vec<_> = rows
                .iter()
                .map(|row| row.get("title").unwrap().clone())
                .collect();

            Ok(Some(rows))
        }
    }
}

pub async fn cargoquery(
    api: &Api,
    tables: &str,
    fields: &str,
    where_condition: Option<&str>,
    join_on: Option<&str>,
) -> Result<Option<Vec<Value>>, Box<dyn std::error::Error>> {
    let mut params = vec![
        ("action", "cargoquery"),
        ("tables", tables),
        ("fields", fields),
        ("limit", "500"),
    ];
    if let Some(where_condition) = where_condition {
        params.push(("where", where_condition));
    };
    if let Some(join_on) = join_on {
        params.push(("join_on", join_on));
    };

    get_json_response(api, &params).await
}

pub async fn get_leagues(
    api: &Api,
    where_condition: Option<&str>,
) -> Result<Option<Vec<Value>>, Box<dyn std::error::Error>> {
    let tables = "Leagues=L";
    let fields = "L.League, L.League_Short, L.Region, L.Level, L.IsOfficial";

    cargoquery(api, tables, fields, where_condition, None).await
}

pub async fn get_tournaments(
    api: &Api,
    where_condition: Option<&str>,
) -> Result<Option<Vec<Value>>, Box<dyn std::error::Error>> {
    let tables = "Leagues=L, Tournaments=T";
    let fields = "T.Name, T.OverviewPage, T.DateStart, T.Date, T.League, T.Region, T.EventType, T.StandardName, T.Split, T.SplitNumber, T.TournamentLevel, T.IsQualifier, T.IsPlayoffs, T.IsOfficial, T.Year";
    let join_on = Some("L.League=T.League");

    cargoquery(api, tables, fields, where_condition, join_on).await
}
