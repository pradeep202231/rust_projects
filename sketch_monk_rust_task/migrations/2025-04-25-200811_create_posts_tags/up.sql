CREATE TABLE posts_tags (
    fk_post_id INTEGER NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    tag VARCHAR(255) NOT NULL,
    PRIMARY KEY (fk_post_id, tag)
);