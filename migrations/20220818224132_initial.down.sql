-- Add down migration script here
DROP INDEX idx_domains;
DROP TABLE domains;

DROP INDEX idx_domain_local_links;
DROP TABLE domain_local_links;

DROP INDEX idx_domain_back_links;
DROP TABLE domain_back_links;