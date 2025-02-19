// Copyright 2022 Zinc Labs Inc. and Contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub async fn get_offset() -> Result<i64, anyhow::Error> {
    let db = &crate::infra::db::DEFAULT;
    let key = "/compact/file_list/offset";
    let value = match db.get(key).await {
        Ok(ret) => String::from_utf8_lossy(&ret).to_string(),
        Err(_) => String::from("0"),
    };
    let offset: i64 = value.parse().unwrap();
    Ok(offset)
}

pub async fn set_offset(offset: i64) -> Result<(), anyhow::Error> {
    let db = &crate::infra::db::DEFAULT;
    let key = "/compact/file_list/offset";
    db.put(key, offset.to_string().into()).await?;
    Ok(())
}

pub async fn set_delete(key: &str) -> Result<(), anyhow::Error> {
    let db = &crate::infra::db::DEFAULT;
    let key = format!("/compact/file_list/delete/{}", key);
    db.put(&key, "OK".into()).await?;
    Ok(())
}

pub async fn del_delete(key: &str) -> Result<(), anyhow::Error> {
    let db = &crate::infra::db::DEFAULT;
    let key = format!("/compact/file_list/delete/{}", key);
    if let Err(e) = db.delete(&key, false).await {
        if !e.to_string().contains("not exists") {
            return Err(anyhow::anyhow!(e));
        }
    }
    Ok(())
}

pub async fn list_delete() -> Result<Vec<String>, anyhow::Error> {
    let mut items = Vec::new();
    let db = &crate::infra::db::DEFAULT;
    let key = "/compact/file_list/delete/";
    let ret = db.list(key).await?;
    for (item_key, _item_value) in ret {
        let item_key = item_key.strip_prefix(key).unwrap();
        items.push(item_key.to_string());
    }
    Ok(items)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[actix_web::test]
    async fn test_files() {
        let off_set = 100;

        let _ = set_offset(off_set).await;
        let resp = get_offset().await;
        assert_eq!(resp.unwrap(), off_set);

        let delete_day = "2023-03-03";
        let _ = set_delete(delete_day).await;
        let deletes = list_delete().await.unwrap();
        assert_eq!([delete_day.to_string()].to_vec(), deletes);
    }
}
