pub struct NamespacedObject {
	pub object_type: String,
	pub object_name: String,
	pub namespace: String,
	pub cluster: String,
}

pub struct ObjectIssue {
	pub object_id: String,
	pub category: String,
	pub details: String,
	pub severity: String,
	pub issue_tech_id: String,
	pub issue_message: String,
	pub reported_by: String,
	pub reported_at: String,
	pub last_seen_at: String,
	pub linked_object_id: String,
}