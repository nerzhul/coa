CREATE TYPE "issue_category" AS ENUM (
	'security',
	'reliability',
	'performance',
	'configuration',
	'unknown'
);

CREATE TYPE "issue_severity" AS ENUM (
	'critical',
	'high',
	'medium',
	'low',
	'unknown'
);

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "namespaced_objects" (
	cluster_name TEXT NOT NULL,
	namespace_name TEXT NOT NULL,
	object_name TEXT NOT NULL,
	object_type TEXT NOT NULL,
	id UUID NOT NULL UNIQUE DEFAULT uuid_generate_v4(),
	CONSTRAINT pkey_issues_objects PRIMARY KEY(object_name, object_type, cluster_name, namespace_name)
);

CREATE TABLE "issues" (
	id UUID NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
	object_id UUID NOT NULL,
	category issue_category NOT NULL,
	details TEXT NOT NULL,
	severity issue_severity NOT NULL,
	issue_tech_id TEXT NOT NULL,
	issue_message TEXT NOT NULL,
	reported_by TEXT NOT NULL,
	reported_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
	last_seen_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
	linked_object_id UUID,

	CONSTRAINT fk_namespaced_objects FOREIGN KEY(object_id) REFERENCES namespaced_objects(id) ON DELETE CASCADE,
	CONSTRAINT fk_issues_subobjobject_id FOREIGN KEY(linked_object_id) REFERENCES namespaced_objects(id) ON DELETE SET NULL(linked_object_id)
);