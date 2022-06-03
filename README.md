# jinwonkim.art

The code for https://jinwonkim.art and it's CMS.

This is a bit of an exercise in cost engineering AWS.

## Structure

```
 +------------------+  +-----------------------------------+
 | cdn (cloudfront) |  |           cms (lambda)            |
 +------------------+  +-----------------------------------+
         |                   |                  |
         V                   V                  V
+--------------------------------------+ +-----------------+
|    www (astro.build output in s3)    | | data (dynamodb) |
+--------------------------------------+ +-----------------+
```

## Design

The initial idea of this is to use services which are on the free forever tier in AWS as much as possible.
