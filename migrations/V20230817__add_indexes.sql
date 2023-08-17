CREATE INDEX idx_namespaced_objects_id_namespace ON namespaced_objects(id, namespace_name);
CREATE INDEX idx_issues_category ON issues(category);