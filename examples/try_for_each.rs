use anyhow::anyhow;

fn main() -> anyhow::Result<()> {
    let data = [Some(123), Some(444), None];

    data.iter().try_for_each(|x| {
        let real_x = x.ok_or(anyhow!(""))?;
        dbg!(real_x);
        anyhow::Ok(())
    })?;

    Ok(())
}
