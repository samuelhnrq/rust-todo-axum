schema "public" {
  comment = "standard public schema"
}

table "users" {
    schema = schema.public
    column "id" {
        type = uuid
        default = sql("uuid_generate_v1mc()")
    }
    column "name" {
        type = varchar
    }
    column "email" {
        type = varchar
    }
    column "createdAt" {
        type = timestamp
    }
    column "updatedAt" {
        type = timestamp
    }
    primary_key {
        columns = [column.id]
    }
}

table "task" {
    schema = schema.public
    column "id" {
        type = uuid
    }
    column "title" {
        type = varchar
    }
    column "description" {
        type = varchar
    }
    column "done" {
        type = boolean
    }
    column "owner_id" {
        type = uuid
    }
    column "createdAt" {
        type = timestamp
    }
    column "updatedAt" {
        type = timestamp
    }
    primary_key {
        columns = [column.id]
    }
    foreign_key "author_fk" {
        columns = [column.owner_id]
        ref_columns = [table.users.column.id]
        on_delete = CASCADE
    }
}
