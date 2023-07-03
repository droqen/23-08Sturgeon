pub const contents : &str = r#"


[gd_scene load_steps=4 format=3 uid="uid://cylhaw0m2kp86"]

[ext_resource type="PackedScene" uid="uid://cw7tc2lmga2h6" path="res://snickers_bar.glb" id="1_r5l1w"]
[ext_resource type="PackedScene" path="res://snickers_pedestal.glb" id="2_ap4vy"]

[sub_resource type="PlaneMesh" id="PlaneMesh_rnsnr"]

[node name="w00_snickers" type="Node3D"]

[node name="plane" type="MeshInstance3D" parent="."]
transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, 0, -0.141755, 0)
mesh = SubResource("PlaneMesh_rnsnr")

[node name="Camera3D" type="Camera3D" parent="."]
transform = Transform3D(0.707107, -0.353553, 0.612372, 0, 0.866025, 0.5, -0.707107, -0.353553, 0.612372, 1, 1, 1)

[node name="snickers_bar4" parent="." instance=ExtResource("1_r5l1w")]
transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, 0.450498, 0, 0.221232)

[node name="snickers_pedestal2" parent="." instance=ExtResource("2_ap4vy")]
transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, 0.450498, 0, 0.221232)


"#;