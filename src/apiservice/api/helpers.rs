use k8s_openapi::api::authorization::v1::SubjectAccessReview;
use kube::api::PostParams;
use kube::core::ObjectMeta;

pub async fn has_rights(kube_client: &kube::Client, namespace: &str, username: &str, groups: &Vec<String>) -> Result<bool, kube::Error> {
	let review_api:  kube::Api<SubjectAccessReview> = kube::Api::all(kube_client.clone());

	let review_result = match review_api.create(&PostParams::default(), &SubjectAccessReview{
		metadata: ObjectMeta::default(),
		spec: k8s_openapi::api::authorization::v1::SubjectAccessReviewSpec{
			resource_attributes: Some(k8s_openapi::api::authorization::v1::ResourceAttributes{
				verb: Some("get".to_string()),
				resource: Some("pods".to_string()),
				group: Some("".to_string()),
				version: Some("v1".to_string()),
				namespace: Some(namespace.to_string()),
				subresource: None,
				name: None,
			}),
			user: Some(username.to_string()),
			groups: Some(groups.to_vec()),
			non_resource_attributes: None,
			extra: None,
			uid: None,
		},
		status: None,
	}).await {
		Ok(r) => r,
		Err(e) => {
			return Err(e);
		}
	};

	Ok(review_result.status.unwrap().allowed)

}

// TODO implement this check
pub fn get_user_context() -> (String, Vec<String>) {
	let username = "admin";
	let groups: Vec<String> = vec!["system:masters".to_string()];

	(username.to_string(), groups)
}