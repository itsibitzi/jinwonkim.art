ALTER TABLE categories ADD COLUMN position INTEGER;

UPDATE categories 
SET position = categories_query.new_position
FROM (
    SELECT 
        id,
        ROW_NUMBER() OVER (ORDER BY id) new_position 
    FROM categories
) categories_query
WHERE categories.id = categories_query.id;

CREATE TRIGGER auto_increment_category_position
    AFTER INSERT ON categories
    WHEN new.position IS NULL
    BEGIN
        UPDATE categories
        SET position = (SELECT IFNULL(MAX(position), 0) + 1 FROM categories)
        WHERE id = new.id;
    END;
